//! 3D incompressible Navier-Stokes on [0,2π)³, pseudo-spectral.
//!
//! Rotational form with the pressure removed by Leray projection:
//!
//!   û_t = P[ FFT(u × ω) ] - ν|k|² û,     P f̂ = f̂ - k (k·f̂)/|k|²
//!
//! 2/3 dealiasing (cube |k_i| <= n/3), integrating-factor RK4, CFL-adaptive dt.
//!
//! Usage: ns3d [n=64] [re=1600] [t_end=20] [ic=tg|abc]
//!
//!   tg  : Taylor-Green vortex (Re=1600 benchmark; dissipation peaks near t≈9)
//!   abc : Arnold-Beltrami-Childress flow with A=B=C=1. It is a Beltrami field
//!         (ω = u, so u×ω = 0): exact solution is u(t) = u0·exp(-ν t).
//!         Use e.g. `ns3d 32 20 5 abc` to validate viscous term / FFT / stepper.
//!
//! Writes <ic>_n<n>.csv with t, E, Z, eps, max|ω|, ∫max|ω|dt (BKM integral).


/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "navier_stokes",
    version: "0.0.1",
    formalized: false,
    notes: "3D pseudo-spectral NS solver, BKM blowup tracking",
};


use rayon::prelude::*;
use rustfft::{num_complex::Complex64 as C, Fft, FftPlanner};
use std::{f64::consts::PI, fs::File, io::Write, sync::Arc};

type Field = Vec<C>;
const ZERO: C = C { re: 0.0, im: 0.0 };

pub struct Solver {
    n: usize,
    fwd: Arc<dyn Fft<f64>>,
    inv: Arc<dyn Fft<f64>>,
    kd: Vec<f64>,  // 1D derivative wavenumbers (Nyquist zeroed)
    k2: Vec<f64>,  // |k|² (integer wavenumbers), n³
    k2p: Vec<f64>, // |k|² built from kd, for the projector, n³
    mask: Vec<f64>,
    nu: f64,
}

fn build<F: Fn(usize) -> C + Sync + Send>(n3: usize, f: F) -> Field {
    (0..n3).into_par_iter().map(f).collect()
}

impl Solver {
    fn new(n: usize, nu: f64) -> Self {
        let mut pl = FftPlanner::new();
        let fwd = pl.plan_fft_forward(n);
        let inv = pl.plan_fft_inverse(n);
        let kint: Vec<f64> = (0..n)
            .map(|i| if i <= n / 2 { i as f64 } else { i as f64 - n as f64 })
            .collect();
        let kd: Vec<f64> = (0..n).map(|i| if 2 * i == n { 0.0 } else { kint[i] }).collect();
        let kmax = (n / 3) as f64;
        let n3 = n * n * n;
        let mut k2 = vec![0.0; n3];
        let mut k2p = vec![0.0; n3];
        let mut mask = vec![0.0; n3];
        for k in 0..n {
            for j in 0..n {
                for i in 0..n {
                    let idx = (k * n + j) * n + i;
                    k2[idx] = kint[i] * kint[i] + kint[j] * kint[j] + kint[k] * kint[k];
                    k2p[idx] = kd[i] * kd[i] + kd[j] * kd[j] + kd[k] * kd[k];
                    if kint[i].abs() <= kmax && kint[j].abs() <= kmax && kint[k].abs() <= kmax {
                        mask[idx] = 1.0;
                    }
                }
            }
        }
        Self { n, fwd, inv, kd, k2, k2p, mask, nu }
    }

    #[inline]
    fn k(&self, idx: usize) -> (f64, f64, f64) {
        let n = self.n;
        (self.kd[idx % n], self.kd[(idx / n) % n], self.kd[idx / (n * n)])
    }

    /// 3D FFT, layout idx = (z*n + y)*n + x. Inverse is scaled by 1/n³.
    fn fft3(&self, a: &mut [C], inverse: bool) {
        let n = self.n;
        let plan = if inverse { &self.inv } else { &self.fwd };
        let sl = plan.get_inplace_scratch_len();

        // x: contiguous rows
        a.par_chunks_mut(n).for_each_init(
            || vec![ZERO; sl],
            |sc, row| plan.process_with_scratch(row, sc),
        );
        // y: strided within each z-slab
        a.par_chunks_mut(n * n).for_each_init(
            || (vec![ZERO; n], vec![ZERO; sl]),
            |(buf, sc), slab| {
                for i in 0..n {
                    for j in 0..n {
                        buf[j] = slab[j * n + i];
                    }
                    plan.process_with_scratch(&mut buf[..], sc);
                    for j in 0..n {
                        slab[j * n + i] = buf[j];
                    }
                }
            },
        );
        // z: swap axes x<->z, transform rows, swap back
        let mut t = vec![ZERO; n * n * n];
        {
            let a_ref: &[C] = &*a;
            t.par_chunks_mut(n * n).enumerate().for_each(|(i, ch)| {
                for j in 0..n {
                    for k in 0..n {
                        ch[j * n + k] = a_ref[(k * n + j) * n + i];
                    }
                }
            });
        }
        t.par_chunks_mut(n).for_each_init(
            || vec![ZERO; sl],
            |sc, row| plan.process_with_scratch(row, sc),
        );
        {
            let t_ref: &[C] = &t;
            a.par_chunks_mut(n * n).enumerate().for_each(|(k, ch)| {
                for j in 0..n {
                    for i in 0..n {
                        ch[j * n + i] = t_ref[(i * n + j) * n + k];
                    }
                }
            });
        }
        if inverse {
            let s = 1.0 / (n * n * n) as f64;
            a.par_iter_mut().for_each(|c| *c *= s);
        }
    }

    fn to_spec(&self, f: &[f64]) -> Field {
        let mut a: Field = f.iter().map(|&x| C::new(x, 0.0)).collect();
        self.fft3(&mut a, false);
        a
    }

    fn to_phys(&self, hat: &[C]) -> Vec<f64> {
        let mut a = hat.to_vec();
        self.fft3(&mut a, true);
        a.par_iter().map(|c| c.re).collect()
    }

    /// ω̂ = i k × û
    fn vorticity_hat(&self, u: &[Field; 3]) -> [Field; 3] {
        let n3 = self.n.pow(3);
        let i = C::new(0.0, 1.0);
        [
            build(n3, |m| { let (_, ky, kz) = self.k(m); i * (u[2][m] * ky - u[1][m] * kz) }),
            build(n3, |m| { let (kx, _, kz) = self.k(m); i * (u[0][m] * kz - u[2][m] * kx) }),
            build(n3, |m| { let (kx, ky, _) = self.k(m); i * (u[1][m] * kx - u[0][m] * ky) }),
        ]
    }

    /// N(û) = P[ mask · FFT(u × ω) ]. Also returns max|u|+max|v|+max|w| for CFL.
    fn nonlinear(&self, uh: &[Field; 3]) -> ([Field; 3], f64) {
        let n3 = self.n.pow(3);
        let oh = self.vorticity_hat(uh);
        let up: Vec<Vec<f64>> = uh.iter().map(|f| self.to_phys(f)).collect();
        let op: Vec<Vec<f64>> = oh.iter().map(|f| self.to_phys(f)).collect();

        let vsum: f64 = up
            .iter()
            .map(|f| f.par_iter().fold(|| 0.0f64, |m, x| m.max(x.abs())).reduce(|| 0.0, f64::max))
            .sum();

        let cross = |a: usize, b: usize, c: usize, d: usize| -> Vec<f64> {
            // u_a ω_b - u_c ω_d
            (0..n3).into_par_iter().map(|m| up[a][m] * op[b][m] - up[c][m] * op[d][m]).collect()
        };
        let mut f0 = self.to_spec(&cross(1, 2, 2, 1)); // v ωz - w ωy
        let mut f1 = self.to_spec(&cross(2, 0, 0, 2)); // w ωx - u ωz
        let mut f2 = self.to_spec(&cross(0, 1, 1, 0)); // u ωy - v ωx

        f0.par_iter_mut()
            .zip(f1.par_iter_mut())
            .zip(f2.par_iter_mut())
            .enumerate()
            .for_each(|(m, ((a, b), c))| {
                let mk = self.mask[m];
                *a *= mk;
                *b *= mk;
                *c *= mk;
                if self.k2p[m] > 0.0 {
                    let (kx, ky, kz) = self.k(m);
                    let kdf = (*a * kx + *b * ky + *c * kz) / self.k2p[m];
                    *a -= kdf * kx;
                    *b -= kdf * ky;
                    *c -= kdf * kz;
                }
            });
        ([f0, f1, f2], vsum)
    }

    /// Integrating-factor RK4. Returns the CFL velocity sum of the *pre-step* state.
    fn step(&self, u: &mut [Field; 3], dt: f64) -> f64 {
        let n3 = self.n.pow(3);
        let e: Vec<f64> = self.k2.par_iter().map(|k| (-self.nu * k * dt).exp()).collect();
        let e2: Vec<f64> = self.k2.par_iter().map(|k| (-self.nu * k * dt / 2.0).exp()).collect();
        let comp3 = |f: &dyn Fn(usize) -> Field| [f(0), f(1), f(2)];

        let (k1, vs) = self.nonlinear(u);
        let a = comp3(&|c| build(n3, |m| e2[m] * (u[c][m] + k1[c][m] * (0.5 * dt))));
        let (k2, _) = self.nonlinear(&a);
        drop(a);
        let b = comp3(&|c| build(n3, |m| e2[m] * u[c][m] + k2[c][m] * (0.5 * dt)));
        let (k3, _) = self.nonlinear(&b);
        drop(b);
        let cc = comp3(&|c| build(n3, |m| e[m] * u[c][m] + k3[c][m] * (dt * e2[m])));
        let (k4, _) = self.nonlinear(&cc);
        drop(cc);

        for c in 0..3 {
            let new = build(n3, |m| {
                e[m] * u[c][m]
                    + (k1[c][m] * e[m] + (k2[c][m] + k3[c][m]) * (2.0 * e2[m]) + k4[c][m]) * (dt / 6.0)
            });
            u[c] = new;
        }
        vs
    }

    fn norm6(&self) -> f64 {
        (self.n as f64).powi(6)
    }

    /// (energy, enstrophy) via Parseval; Z = ½Σ|k|²|û|² since |ω̂|² = |k|²|û|².
    fn energy_enstrophy(&self, u: &[Field; 3]) -> (f64, f64) {
        let (e, z) = (0..self.n.pow(3))
            .into_par_iter()
            .map(|m| {
                let p = u[0][m].norm_sqr() + u[1][m].norm_sqr() + u[2][m].norm_sqr();
                (0.5 * p, 0.5 * p * self.k2[m])
            })
            .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
        (e / self.norm6(), z / self.norm6())
    }

    fn wmax(&self, u: &[Field; 3]) -> f64 {
        let oh = self.vorticity_hat(u);
        let op: Vec<Vec<f64>> = oh.iter().map(|f| self.to_phys(f)).collect();
        (0..self.n.pow(3))
            .into_par_iter()
            .map(|m| (op[0][m].powi(2) + op[1][m].powi(2) + op[2][m].powi(2)).sqrt())
            .reduce(|| 0.0, f64::max)
    }

    fn div_max(&self, u: &[Field; 3]) -> f64 {
        let i = C::new(0.0, 1.0);
        (0..self.n.pow(3))
            .into_par_iter()
            .map(|m| {
                let (kx, ky, kz) = self.k(m);
                (i * (u[0][m] * kx + u[1][m] * ky + u[2][m] * kz)).norm()
            })
            .reduce(|| 0.0, f64::max)
    }

    /// Fraction of enstrophy in the top quarter of retained |k| — under-resolution alarm.
    fn tail_fraction(&self, u: &[Field; 3]) -> f64 {
        let kc = (self.n / 3) as f64;
        let cut = (0.75 * kc).powi(2);
        let (tail, tot) = (0..self.n.pow(3))
            .into_par_iter()
            .map(|m| {
                let p = (u[0][m].norm_sqr() + u[1][m].norm_sqr() + u[2][m].norm_sqr()) * self.k2[m];
                (if self.k2[m] > cut { p } else { 0.0 }, p)
            })
            .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
        if tot > 0.0 { tail / tot } else { 0.0 }
    }

    fn initial(&self, ic: &str) -> [Field; 3] {
        let n = self.n;
        let dx = 2.0 * PI / n as f64;
        let n3 = n * n * n;
        let mut p = [vec![0.0; n3], vec![0.0; n3], vec![0.0; n3]];
        for k in 0..n {
            for j in 0..n {
                for i in 0..n {
                    let idx = (k * n + j) * n + i;
                    let (x, y, z) = (i as f64 * dx, j as f64 * dx, k as f64 * dx);
                    match ic {
                        "abc" => {
                            p[0][idx] = z.sin() + y.cos();
                            p[1][idx] = x.sin() + z.cos();
                            p[2][idx] = y.sin() + x.cos();
                        }
                        _ => {
                            p[0][idx] = x.sin() * y.cos() * z.cos();
                            p[1][idx] = -x.cos() * y.sin() * z.cos();
                            p[2][idx] = 0.0;
                        }
                    }
                }
            }
        }
        let mut u = [self.to_spec(&p[0]), self.to_spec(&p[1]), self.to_spec(&p[2])];
        for c in 0..3 {
            u[c].par_iter_mut().zip(self.mask.par_iter()).for_each(|(a, m)| *a *= *m);
        }
        u
    }
}


/// Run a Navier-Stokes simulation and return results.
/// 
/// # Arguments
/// * `n` - Grid resolution (n³ grid points)
/// * `re` - Reynolds number
/// * `t_end` - End time
/// * `ic` - Initial condition: "tg" (Taylor-Green vortex) or "abc" (Arnold-Beltrami-Childress)
/// 
/// # Returns
/// Simulation results including energy, enstrophy, BKM integral, etc.
pub fn run_simulation(n: usize, re: f64, t_end: f64, ic: &str) -> SimulationResult {
    let nu = 1.0 / re;
    let s = Solver::new(n, nu);
    let dx = 2.0 * PI / n as f64;
    let mut u = s.initial(ic);

    let (e0, z0) = s.energy_enstrophy(&u);
    let mut vsum = s.nonlinear(&u).1;

    let csv_name = format!("{ic}_n{n}.csv");
    let mut csv = File::create(&csv_name).expect("csv");
    writeln!(csv, "t,E,Z,eps,wmax,bkm").unwrap();

    let (cfl, dt_max) = (0.5, 0.05);
    let (mut t, mut steps) = (0.0f64, 0usize);
    let mut wm = s.wmax(&u);
    let mut bkm = 0.0;
    let (mut next_csv, _) = (0.0, 0.0);
    let (mut eps_peak, mut t_peak) = (0.0f64, 0.0f64);

    loop {
        let (e, z) = s.energy_enstrophy(&u);
        let eps = 2.0 * nu * z;
        if eps > eps_peak {
            eps_peak = eps;
            t_peak = t;
        }
        let mut dt = (cfl * dx / vsum.max(1e-12)).min(dt_max);
        let done = t + dt >= t_end;
        if done {
            dt = t_end - t;
        }
        if t >= next_csv || done {
            writeln!(csv, "{t},{e},{z},{eps},{wm},{bkm}").unwrap();
            next_csv += 0.25;
        }
        vsum = s.step(&mut u, dt);
        t += dt;
        steps += 1;
        let w_new = s.wmax(&u);
        bkm += 0.5 * (w_new + wm) * dt;
        wm = w_new;
        if done {
            break;
        }
    }

    let (e, z) = s.energy_enstrophy(&u);
    drop(csv);

    SimulationResult {
        n, re, t_end: t, ic: ic.to_string(),
        e0, z0, e_final: e, z_final: z,
        steps, bkm_integral: bkm,
        eps_peak, t_peak,
        divergence_max: s.div_max(&u),
        csv_file: csv_name,
    }
}

/// Results from a Navier-Stokes simulation.
#[derive(Debug)]
pub struct SimulationResult {
    pub n: usize,
    pub re: f64,
    pub t_end: f64,
    pub ic: String,
    pub e0: f64,
    pub z0: f64,
    pub e_final: f64,
    pub z_final: f64,
    pub steps: usize,
    pub bkm_integral: f64,
    pub eps_peak: f64,
    pub t_peak: f64,
    pub divergence_max: f64,
    pub csv_file: String,
}

impl std::fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Navier-Stokes Simulation Results")?;
        writeln!(f, "  Grid: {}³ ({} points)", self.n, self.n.pow(3))?;
        writeln!(f, "  Reynolds number: {}", self.re)?;
        writeln!(f, "  Initial condition: {}", self.ic)?;
        writeln!(f, "  Time: 0 → {:.4}", self.t_end)?;
        writeln!(f, "  Steps: {}", self.steps)?;
        writeln!(f, "")?;
        writeln!(f, "Energy:")?;
        writeln!(f, "  E₀ = {:.6}", self.e0)?;
        writeln!(f, "  E(t_end) = {:.6} ({:.2}% of initial)", self.e_final, self.e_final / self.e0 * 100.0)?;
        if self.ic == "abc" {
            let exact = (-2.0 * (1.0 / self.re) * self.t_end).exp();
            writeln!(f, "  Exact decay: {:.6} (rel err: {:.2e})", exact, ((self.e_final / self.e0) - exact).abs() / exact)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Enstrophy:")?;
        writeln!(f, "  Z₀ = {:.6}", self.z0)?;
        writeln!(f, "  Z(t_end) = {:.6} ({:.2}% of initial)", self.z_final, self.z_final / self.z0 * 100.0)?;
        writeln!(f, "")?;
        writeln!(f, "Dissipation:")?;
        writeln!(f, "  Peak ε = {:.5e} at t = {:.3}", self.eps_peak, self.t_peak)?;
        writeln!(f, "")?;
        writeln!(f, "BKM integral ∫max|ω|dt = {:.4}", self.bkm_integral)?;
        writeln!(f, "Max divergence |k·û| = {:.2e}", self.divergence_max)?;
        writeln!(f, "Data: {}", self.csv_file)?;
        Ok(())
    }
}


/// Educational station for the main binary.
pub fn show_navier_stokes_station() {
    println!("=== NAVIER-STOKES MILLENNIUM PRIZE PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\n");

    println!("THE QUESTION: Do smooth, initial-data solutions to the 3D incompressible");
    println!("Navier-Stokes equations remain smooth for all time, or can they develop");
    println!("singularities (blow up) in finite time?\n");

    println!("The equations describe how velocity u and pressure p evolve:");
    println!("  ∂u/∂t + (u·∇)u = -∇p + ν∇²u,     ∇·u = 0\n");

    println!("We simulate this using pseudo-spectral methods on [0,2π)³:");
    println!("  • Transform to Fourier space with FFT");
    println!("  • Remove pressure via Leray projection (enforces incompressibility)");
    println!("  • Advance with integrating-factor RK4 + CFL-adaptive timestep\n");

    println!("Running a short validation simulation (ABC flow, n=32, Re=1600, t=5):");
    println!("The ABC flow is a Beltrami field (ω = u), so it has an exact solution:");
    println!("  E(t) = E₀ · exp(-2νt)\n");

    let result = run_simulation(32, 1600.0, 5.0, "abc");
    println!("{}", result);

    println!();
    println!("THE BKM CRITERION (Beale-Kato-Majda):");
    println!("If the integral ∫₀ᵀ max|ω(t)| dt diverges as T → T*, then the solution");
    println!("blows up at time T*. Our simulation tracks this integral.\n");

    println!("Current state of the problem:");
    println!("  • No blowup has been found numerically (simulations stay smooth)");
    println!("  • No proof exists that blowup cannot happen");
    println!("  • The equations are well-posed in 2D but open in 3D");
    println!("  • This is one of the seven Millennium Prize Problems\n");

    println!("For a longer run: cargo run --release --example ns3d 64 1600 20 tg");
}
