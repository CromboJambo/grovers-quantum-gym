//! Navier-Stokes Millennium Prize Problem Station
//!
//! Explores the Clay Mathematics Institute's $1 million question:
//! Do smooth solutions to the 3D incompressible Navier-Stokes equations
//! always exist, or can they develop singularities (blow up) in finite time?
//!
//! Demonstrates via a 2D projection method solver with viscosity.

const GRID: usize = 32;
const DT: f64 = 0.01;
const VISCOSITY: f64 = 0.001;

/// Simple Jacobi iteration for Poisson equation: ∇²φ = f
fn solve_poisson(f: &[f64], n: usize, iters: usize) -> Vec<f64> {
    let mut phi = vec![0.0; n * n];

    for _ in 0..iters {
        let mut new_phi = phi.clone();
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let idx = j * n + i;
                new_phi[idx] =
                    (f[idx] + phi[idx - 1] + phi[idx + 1] + phi[idx - n] + phi[idx + n]) / 4.0;
            }
        }
        phi = new_phi;
    }

    phi
}

/// Apply diffusion operator: ∂u/∂t = ν∇²u (explicit scheme)
fn diffuse(field: &[f64], n: usize, nu: f64, dt: f64) -> Vec<f64> {
    let mut result = field.to_vec();

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            let laplacian = field[idx - 1] + field[idx + 1] + field[idx - n] + field[idx + n]
                - 4.0 * field[idx];
            result[idx] = field[idx] + nu * dt * laplacian;
        }
    }

    result
}

/// Advection step: ∂u/∂t + u·∇u = 0 (semi-Lagrangian, simplified)
fn advect(field: &[f64], vel_x: &[f64], vel_y: &[f64], n: usize, dt: f64) -> Vec<f64> {
    let mut result = field.to_vec();

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            // Backtrace particle position (simplified - just use current velocity)
            let back_i = i as f64 - vel_x[idx] * dt;
            let back_j = j as f64 - vel_y[idx] * dt;

            if back_i > 0.0 && back_i < n as f64 && back_j > 0.0 && back_j < n as f64 {
                // Bilinear interpolation (simplified to nearest)
                let src_i = back_i as usize;
                let src_j = back_j as usize;
                result[idx] = field[src_j * n + src_i];
            }
        }
    }

    result
}

/// Project velocity onto divergence-free field (Helmholtz decomposition)
fn project(vel_x: &[f64], vel_y: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    // Compute divergence
    let mut div = vec![0.0; n * n];
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            div[idx] =
                (vel_x[idx + 1] - vel_x[idx - 1]) / 2.0 + (vel_y[idx + n] - vel_y[idx - n]) / 2.0;
        }
    }

    // Solve for pressure: ∇²p = div
    let p = solve_poisson(&div, n, 100);

    // Subtract gradient of pressure
    let mut new_vel_x = vel_x.to_vec();
    let mut new_vel_y = vel_y.to_vec();
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            new_vel_x[idx] -= (p[idx + 1] - p[idx - 1]) / 2.0;
            new_vel_y[idx] -= (p[idx + n] - p[idx - n]) / 2.0;
        }
    }

    (new_vel_x, new_vel_y)
}

/// Initialize flow with a vortex
fn init_vortex(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut vel_x = vec![0.0; n * n];
    let mut vel_y = vec![0.0; n * n];

    let center = n as f64 / 2.0;
    for j in 0..n {
        for i in 0..n {
            let idx = j * n + i;
            let dx = i as f64 - center;
            let dy = j as f64 - center;
            let r2 = dx * dx + dy * dy;

            if r2 > 1.0 && r2 < (n as f64 * n as f64) / 4.0 {
                // Rankine vortex: solid body rotation inside, decay outside
                vel_x[idx] = -dy / (r2 + 1.0);
                vel_y[idx] = dx / (r2 + 1.0);
            }
        }
    }

    (vel_x, vel_y)
}

/// Demonstrate energy cascade: how nonlinear advection transfers energy
/// from large scales to small scales (potential blowup mechanism)
fn demonstrate_energy_cascade() {
    println!("--- ENERGY CASCADE DEMONSTRATION ---");
    println!();
    println!("The key question: Can nonlinear advection concentrate energy");
    println!("at smaller scales faster than viscosity can dissipate it?");
    println!();

    // Simulate a simplified energy cascade across "scales"
    let n_scales = 10;
    let mut energy = vec![0.0; n_scales];
    energy[0] = 1.0; // All energy starts at largest scale

    println!("Scale | Energy | Notes");
    println!("------|--------|------");

    let dt_cascade = 0.1;
    for step in 0..20 {
        // Nonlinear transfer: energy moves from large to small scales
        // This is the (u·∇)u term doing its work
        let mut transfers = vec![0.0; n_scales];
        for s in 0..n_scales - 1 {
            // Energy transfer rate proportional to energy at this scale
            let transfer = energy[s] * dt_cascade * (s + 1) as f64 / 5.0;
            transfers[s] -= transfer;
            transfers[s + 1] += transfer;
        }

        // Viscous dissipation: stronger at small scales
        for s in 0..n_scales {
            let viscosity = 0.01 * (s + 1) as f64;
            energy[s] -= viscosity * energy[s] * dt_cascade;
            energy[s] += transfers[s];
            if energy[s] < 0.0 {
                energy[s] = 0.0;
            }
        }

        if step == 0 || step == 5 || step == 10 || step == 19 {
            let total: f64 = energy.iter().sum();
            println!("Step {:2}: total_energy={:.4}", step, total);

            // Show distribution
            for s in 0..n_scales {
                if energy[s] > 0.001 {
                    let note = if s == 0 {
                        "Largest scales (input)"
                    } else if s == n_scales - 1 {
                        "Smallest scales (dissipation)"
                    } else {
                        ""
                    };
                    println!("      {:5}: {:.6} {}", s, energy[s], note);
                }
            }
            println!();
        }
    }

    let total_final: f64 = energy.iter().sum();
    println!(
        "Final total energy: {:.4} (dissipated {:.1}%)",
        total_final,
        (1.0 - total_final) * 100.0
    );
    println!();
    println!("In this demo, viscosity wins — energy dissipates.");
    println!("But in 3D with stronger nonlinearity, could the cascade");
    println!("accelerate and create infinite velocity in finite time?");
    println!("That's the Clay problem. Nobody knows.\n");
}

/// Compute vorticity magnitude at each grid point
fn compute_vorticity(vel_x: &[f64], vel_y: &[f64], n: usize) -> Vec<f64> {
    let mut omega = vec![0.0; n * n];

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            // ω = ∂v/∂x - ∂u/∂y (2D vorticity)
            let dvdx = (vel_y[idx + 1] - vel_y[idx - 1]) / 2.0;
            let dudy = (vel_x[idx + n] - vel_x[idx - n]) / 2.0;
            omega[idx] = dvdx - dudy;
        }
    }

    omega
}

/// Compute total kinetic energy of the flow
fn kinetic_energy(vel_x: &[f64], vel_y: &[f64], n: usize) -> f64 {
    let mut e = 0.0;
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let idx = j * n + i;
            e += vel_x[idx] * vel_x[idx] + vel_y[idx] * vel_y[idx];
        }
    }
    e / (n as f64 * n as f64)
}

pub fn show_navier_stokes_station() {
    println!("=== NAVIER-STOKES MILLENNIUM PRIZE PROBLEM ===");
    println!("Clay Mathematics Institute: $1,000,000 prize for proof\n");

    println!("THE QUESTION: Do smooth solutions to the 3D incompressible");
    println!("Navier-Stokes equations always exist? Or can they develop");
    println!("singularities (infinite velocity) in finite time?\n");

    println!("The equations describe fluid motion:");
    println!("∂u/∂t + (u·∇)u = -∇p + ν∇²u + f  (momentum)");
    println!("∇·u = 0                              (incompressibility)\n");

    println!("Where u = velocity, p = pressure, ν = viscosity\n");

    println!("--- EXPERIMENT: VORTEX DECAY ---");
    println!(
        "Simulating a decaying vortex on a {}x{} grid for 100 time steps.",
        GRID, GRID
    );
    println!("Viscosity ν = {:.4}\n", VISCOSITY);

    // Initialize vortex
    let (mut vel_x, mut vel_y) = init_vortex(GRID);

    let initial_energy = kinetic_energy(&vel_x, &vel_y, GRID);
    let initial_omega = compute_vorticity(&vel_x, &vel_y, GRID);
    let max_initial_omega = initial_omega
        .iter()
        .fold(0.0f64, |m, &v| if v.abs() > m.abs() { v } else { m });

    println!("Initial kinetic energy: {:.6}", initial_energy);
    println!("Initial max vorticity:  {:.4}\n", max_initial_omega);

    // Time stepping
    for step in 0..100 {
        // Diffusion (viscosity smooths)
        let vel_x_diff = diffuse(&vel_x, GRID, VISCOSITY, DT);
        let vel_y_diff = diffuse(&vel_y, GRID, VISCOSITY, DT);

        // Advect (nonlinear self-interaction - source of potential blowup)
        let vel_x_adv = advect(&vel_x_diff, &vel_x_diff, &vel_y_diff, GRID, DT);
        let vel_y_adv = advect(&vel_y_diff, &vel_x_diff, &vel_y_diff, GRID, DT);

        // Project onto divergence-free field (enforce incompressibility)
        (vel_x, vel_y) = project(&vel_x_adv, &vel_y_adv, GRID);

        if step == 25 || step == 50 || step == 75 || step == 99 {
            let energy = kinetic_energy(&vel_x, &vel_y, GRID);
            let omega = compute_vorticity(&vel_x, &vel_y, GRID);
            let max_omega = omega
                .iter()
                .fold(0.0f64, |m, &v| if v.abs() > m.abs() { v } else { m });

            println!(
                "Step {:3}: energy={:.6}  max_vorticity={:.4}",
                step + 1,
                energy,
                max_omega
            );
        }
    }

    let final_energy = kinetic_energy(&vel_x, &vel_y, GRID);
    let final_omega = compute_vorticity(&vel_x, &vel_y, GRID);
    let max_final_omega = final_omega
        .iter()
        .fold(0.0f64, |m, &v| if v.abs() > m.abs() { v } else { m });

    println!(
        "\nFinal kinetic energy:   {:.6} (dissipated {:.1}%)",
        final_energy,
        (1.0 - final_energy / initial_energy) * 100.0
    );
    println!("Final max vorticity:    {:.4}", max_final_omega);

    println!("--- WHAT THIS SHOWS ---");
    println!("In this viscous simulation, the vortex decays smoothly.");
    println!("Kinetic energy is dissipated by viscosity (ν∇²u term).");
    println!("No singularity forms - solution remains smooth.\n");

    println!("BUT: The Clay problem asks about 3D solutions for ALL time.");
    println!("In 2D, global existence and uniqueness are proven. In 3D:");
    println!("- The nonlinear advection term (u·∇)u can potentially amplify");
    println!("  velocity gradients faster than viscosity can smooth them.");
    println!("- If this happens, vorticity could become infinite in finite time.");
    println!("- No one has proven whether this can actually happen.\n");

    println!("--- OPENAI'S CLAIMED SOLUTION (Sept 2026) ---");
    println!("OpenAI claims to have CONSTRUCTED a solution that blows up.");
    println!("Their approach: design oscillatory initial conditions where");
    println!("nonlinear advection creates a self-reinforcing cascade:");
    println!("1. Velocity gradients focus energy at smaller scales");
    println!("2. At smaller scales, viscosity is weaker (∝ 1/scale²)");
    println!("3. Nonlinear term grows faster than viscous damping");
    println!("4. Singularity forms in finite time\n");
    println!("This is the energy cascade taken to its logical extreme —");
    println!("exactly what this station demonstrates conceptually above.");
    println!("\n--- NUMERICAL VERIFICATION TOOL ---");
    println!("The ns3d/ directory contains a pseudo-spectral 3D Navier-Stokes");
    println!("solver built to numerically test blowup claims. Usage:");
    println!("  cd ns3d && cargo run --release -- [n] [re] [t_end] [ic]");
    println!("Initial conditions: 'tg' (Taylor-Green vortex) or 'abc' (Beltrami flow)");
    println!("The solver validates itself against known exact solutions and");
    println!("tracks the Beale-Kato-Majama integral ∫max|ω|dt as a blowup indicator.");

    println!("\n--- WHY IT'S HARD ---");
    println!("- Navier-Stokes is nonlinear: solutions interact with themselves");
    println!("- Small-scale structures cascade to smaller scales (turbulence)");
    println!("- We don't know if energy always cascades down or can concentrate");
    println!("  into singularities\n");

    println!("--- CONNECTION TO TURBULENCE ---");
    println!("Turbulent flow is the most complex motion of matter known.");
    println!("The transition from laminar to turbulent flow remains one of");
    println!("the great unsolved problems in physics. If singularities can");
    println!("form, it would explain why turbulence is so chaotic and hard");
    println!("to predict.\n");

    println!("=== NAVIER-STOKES STATION COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_solver() {
        // Simple test: ∇²φ = -4πδ(x) → φ should be positive near center
        let n = 10;
        let mut f = vec![0.0; n * n];
        f[n / 2 * n + n / 2] = 1.0; // Point source

        let phi = solve_poisson(&f, n, 100);

        // Solution should be positive near source
        assert!(phi[n / 2 * n + n / 2] > 0.0);
    }

    #[test]
    fn test_diffusion_smooths() {
        let n = 10;
        let mut field = vec![0.0; n * n];
        field[n / 2 * n + n / 2] = 1.0; // Hot spot

        let diffused = diffuse(&field, n, 0.1, 1.0);

        // Diffusion should spread the heat
        assert!(diffused[n / 2 * n + n / 2] < 1.0);
        assert!(diffused[n / 2 * n + n / 2 - 1] > 0.0);
    }

    #[test]
    fn test_vorticity_computation() {
        let n = 10;
        let mut vel_x = vec![0.0; n * n];
        let vel_y = vec![0.0; n * n];

        // Simple shear flow: u = (y, 0) → ω = -1 everywhere
        for j in 0..n {
            for i in 0..n {
                vel_x[j * n + i] = j as f64;
            }
        }

        let omega = compute_vorticity(&vel_x, &vel_y, n);

        // Interior points should have ω ≈ -1
        assert!((omega[n / 2 * n + n / 2] + 1.0).abs() < 0.1);
    }

    #[test]
    fn test_kinetic_energy_positive() {
        let n = 10;
        let mut vel_x = vec![0.0; n * n];
        let mut vel_y = vec![0.0; n * n];

        // Uniform flow
        for idx in 0..n * n {
            vel_x[idx] = 1.0;
            vel_y[idx] = 1.0;
        }

        let e = kinetic_energy(&vel_x, &vel_y, n);
        assert!(e > 0.0);
    }
}
