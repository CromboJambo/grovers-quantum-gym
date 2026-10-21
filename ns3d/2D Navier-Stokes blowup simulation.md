---
title: "2D Navier-Stokes blowup simulation"
source: "https://claude.ai/chat/9e046ecd-a898-48fb-b3f8-2d210281c6d9"
author:
published: 2026-10-19
created: 2026-10-21
description: "Claude conversation with 3 messages"
tags:
  - "clippings"
---


[

Claude

](/new)

[NewCtrl+Shift+O](/new)

[Projects](/projects)

[Artifacts](/artifacts)[CodeUpgrade](/upgrade?feature=code&returnTo=%2Fcode)[Customize](/customize)

Pinned

Drag to pin

Chats and tasks



[2D Navier-Stokes blowup simulation](/chat/9e046ecd-a898-48fb-b3f8-2d210281c6d9)



[LLM runner in Rust performance comparison](/chat/3ffee868-96d4-4551-9b6e-c12f22ab2bfb)



[Structural tokenization analysis for herdr workspace](/chat/7cef4332-5bc6-496e-b342-31dbedddafbb)



[Mathematical notation exploration](/chat/e848f60c-4be2-4e0c-b2a2-02ad991a1300)



[Grandma's ratatouille blackjack card counting](/chat/3ef983a6-8b09-4d38-b2e9-06f1006a8152)



[Natural logarithm of zero](/chat/f63e60c8-04b3-440a-a696-9cf766f02667)



[Parser code review](/chat/99bbbe47-55b2-4b25-828f-9701f76cdc6d)



[Building a human-in-the-loop Rust agent](/chat/817312ee-cc96-4320-a726-c94e2ed8b29d)



[Rebuilding Teams electron wrapper in rust](/chat/3b45a0bd-390e-448b-a59c-c468647856ce)



[Debugging GGUF tensor offset parsing issue](/chat/b9a16b77-627e-4b5d-a57a-40c3d90701a5)



[Rust-based VM serving over Tailscale](/chat/65475a8d-c3d7-4170-89f5-2be8911b7658)



[PESTI workspace acronym guess](/chat/d740c46c-c02e-42d7-8638-b82e752472d5)



[Container imaging for Rust cargo packages](/chat/9da7166a-7bfd-443a-8bb2-27bedca27bdb)



[Rotating API keys complexity](/chat/477b399c-da91-424f-ad95-36abca879740)



[Inbox optimization strategy](/chat/9654fa1a-9dac-4c6c-bb97-256fdfac43fb)



[Untitled](/chat/cfb9b45a-01e8-4988-87d3-13c65d7f687b)



[Message size limitations and content truncation](/chat/3aa166a9-f026-4ac1-85f5-0a22c812c363)



[Message size limitations and content truncation](/chat/6cf2525d-b61a-4a8c-86b1-c98b152bb262)



[Agent progress on streaming implementation with lifetime constraints](/chat/b9d30b94-f248-4c25-a434-8321bc94a0fa)



[GPU kernel compiled in pure Rust](/chat/dc92e27d-b5b8-411a-8e1d-7383e2335038)



View all

Sessions you start will show up here

DGDustin·Free

[](/downloads)

2D Navier-Stokes blowup simulation

Free plan

[Upgrade](/upgrade)

Share

Use the up and down arrow keys to move between messages.

## You said: // 2D incompressible Navier-Stokes solver with oscillatory initial conditions

// 2D incompressible Navier-Stokes solver with oscillatory initial conditions  
// Tests OpenAI's claimed blowup construction numerically.  
//  
// Initial conditions: u = sin(10x)\*cos(5y), v = -cos(10x)\*sin(5y)  
// Parameters: dt=0.001, nu=0.01, grid 64x64, 1000 time steps

const NX: usize = 64;  
const NY: usize = 64;  
const DT: f64 = 0.0001;  
const NU: f64 = 0.01;  
const STEPS: i32 = 1000;

fn main() {  
let dx = 2.0 \* std::f64::consts::PI / (NX as f64);  
let dy = 2.0 \* std::f64::consts::PI / (NY as f64);

// Allocate velocity fields (u, v) and their updates  
let mut u = vec!\[0.0; NX \* NY\];  
let mut v = vec!\[0.0; NX \* NY\];  
let mut u\_new = vec!\[0.0; NX \* NY\];  
let mut v\_new = vec!\[0.0; NX \* NY\];

// Initial conditions: oscillatory velocity field  
for j in 0..NY {  
for i in 0..NX {  
let x = (i as f64) \* dx;  
let y = (j as f64) \* dy;  
u\[i + j \* NX\] = (10.0 \* x).sin() \* (5.0 \* y).cos();  
v\[i + j \* NX\] = -(10.0 \* x).cos() \* (5.0 \* y).sin();  
}  
}

let mut max\_vel = 0.0;  
let mut step\_max = \[0.0; STEPS as usize\];

// Compute initial max velocity  
for j in 0..NY {  
for i in 0..NX {  
let idx = i + j \* NX;  
let vel\_sq = u\[idx\] \* u\[idx\] + v\[idx\] \* v\[idx\];  
if vel\_sq > max\_vel \* max\_vel {  
max\_vel = vel\_sq.sqrt();  
}  
}  
}  
step\_max\[0\] = max\_vel;  
println!("Step 0: max velocity = {:.6}", max\_vel);

// Time stepping loop (explicit Euler)  
for step in 1..STEPS {  
// Compute Laplacian and advection terms  
for j in 0..NY {  
for i in 0..NX {  
let idx = i + j \* NX;

// Periodic neighbor indices  
let ip = (i + 1) % NX;  
let im = if i == 0 { NX - 1 } else { i - 1 };  
let jp = (j + 1) % NY;  
let jm = if j == 0 { NY - 1 } else { j - 1 };

// Laplacian of u: ∇²u ≈ (u\_{i+1,j} - 2u\_{i,j} + u\_{i-1,j})/dx² + ...  
let lap\_u = ((u\[ip + j \* NX\] - 2.0 \* u\[idx\] + u\[im + j \* NX\]) / (dx \* dx))  
\+ ((u\[i + jp \* NX\] - 2.0 \* u\[idx\] + u\[i + jm \* NX\]) / (dy \* dy));

// Laplacian of v: ∇²v ≈ ...  
let lap\_v = ((v\[ip + j \* NX\] - 2.0 \* v\[idx\] + v\[im + j \* NX\]) / (dx \* dx))  
\+ ((v\[i + jp \* NX\] - 2.0 \* v\[idx\] + v\[i + jm \* NX\]) / (dy \* dy));

// Advection terms: u·∇u and u·∇v (upwind differencing)  
let du\_dx = (u\[ip + j \* NX\] - u\[im + j \* NX\]) / (2.0 \* dx);  
let du\_dy = (u\[i + jp \* NX\] - u\[i + jm \* NX\]) / (2.0 \* dy);  
let dv\_dx = (v\[ip + j \* NX\] - v\[im + j \* NX\]) / (2.0 \* dx);  
let dv\_dy = (v\[i + jp \* NX\] - v\[i + jm \* NX\]) / (2.0 \* dy);

// Explicit Euler update: u\_new = u - dt\*(u·∇u) + dt\*nu\*∇²u  
u\_new\[idx\] = u\[idx\] - DT \* (u\[idx\] \* du\_dx + v\[idx\] \* du\_dy) + DT \* NU \* lap\_u;  
v\_new\[idx\] = v\[idx\] - DT \* (u\[idx\] \* dv\_dx + v\[idx\] \* dv\_dy) + DT \* NU \* lap\_v;  
}  
}

// Swap buffers  
std::mem::swap(&mut u, &mut u\_new);  
std::mem::swap(&mut v, &mut v\_new);

// Compute max velocity at this step  
let mut step\_max\_vel = 0.0;  
for j in 0..NY {  
for i in 0..NX {  
let idx = i + j \* NX;  
let vel\_sq = u\[idx\] \* u\[idx\] + v\[idx\] \* v\[idx\];  
if vel\_sq > step\_max\_vel \* step\_max\_vel {  
step\_max\_vel = vel\_sq.sqrt();  
}  
}  
}  
step\_max\[step as usize\] = step\_max\_vel;

// Report every 100 steps  
if step % 100 == 0 || step == STEPS - 1 {  
println!("Step {}: max velocity = {:.6}", step, step\_max\_vel);  
}  
}

// Determine blowup: compare final max velocity to initial  
let blowup = step\_max\[STEPS as usize - 1\] > step\_max\[0\] \* 2.0;  
println!("\\n=== RESULTS ===");  
println!("Initial max velocity: {:.6}", step\_max\[0\]);  
println!("Final max velocity: {:.6}", step\_max\[STEPS as usize - 1\]);  
println!("Blowup observed (2x growth): {}", blowup);

// Write JSON result to stdout for parsing  
println!("\\n{{\\"blowup\_observed\\":{},\\"max\_velocity\_reached\\":{:.6}}}",  
blowup, step\_max\[STEPS as usize - 1\]);  
}

Message collapsedShow more

yesterday

## Claude responded: This code can't test a blowup claim, and the likely output is simple viscous decay.

Checking for stability bugs and incompressible flow violations.

Checking for stability bugs and incompressible flow violations.

This code can’t test a blowup claim, and the likely output is simple viscous decay. The main problems:

**1\. It isn’t incompressible Navier-Stokes.**

- There is no pressure term or projection step, so the solver is really viscous Burgers.
- The initial field isn’t divergence-free either: ∂u/∂x + ∂v/∂y = 10cos(10x)cos(5y) − 5cos(10x)cos(5y) = 5cos(10x)cos(5y) ≠ 0.
- A divergence-free version would use a stream function, for example u = ∂ψ/∂y and v = −∂ψ/∂x.

**2\. 2D can’t blow up.**

- Smooth data for 2D Navier-Stokes (and 2D Euler) stays smooth for all time. This is classical, going back to Ladyzhenskaya.
- Any growth you see in 2D is numerical error or the missing pressure term. Any blowup construction of interest would have to be 3D (or an axisymmetric-with-swirl or boundary setup). I can’t verify what OpenAI actually claimed, so check which setting it was.

**3\. The run is too short and mild to show anything.**

- DT is 1e-4, not the 1e-3 in your header comment, so 1000 steps covers t = 0.1.
- Viscous decay for wavenumbers (10, 5) is about exp(−ν·125·t) ≈ exp(−0.125) ≈ 0.88.
- CFL and diffusion numbers are around 1e-3 and 1e-4, so the scheme is stable. I’d expect “final max velocity ≈ 0.88, blowup: false”.
- That result would say nothing about the claim.

**4\. Smaller issues.**

- The comment says “upwind differencing”, but the code uses central differences.
- Wavenumber 10 on 64 points is only about 6.4 points per wavelength, so central-difference error is large.
- Max velocity is the wrong diagnostic. Blowup criteria (Beale-Kato-Majda) concern ∫‖ω‖∞ dt, so track max vorticity and enstrophy.
- A 2× growth threshold in max |u| is arbitrary. For viscous flow, |u|max is bounded for a long time even when vorticity gets wild.

**What a meaningful test needs:**

- Use a pseudo-spectral method with 2/3 dealiasing and Leray projection (û ← û − k(k·û)/|k|²), RK4 time stepping, and adaptive dt from CFL.
- Work in 3D, with the exact initial data from the claimed construction.
- Run a resolution refinement study (128³ → 256³ → 512³), and track max |ω|, enstrophy, and the analyticity-strip width. Blowup shows as ‖ω‖∞ diverging with a consistent power law (like 1/(T−t)) that persists under refinement, not as instability at one resolution.
- Even then, a numerical run can only suggest blowup, not prove it. Computer-assisted proofs need interval arithmetic and rigorous bounds.

I can sketch a Rust pseudo-spectral solver (e.g. using `rustfft`) with Leray projection and vorticity diagnostics, starting in 2D as a sanity check and then extending to 3D.

yesterday

## You said: if you insist.

if you insist. I certainly would enjoy that!

yesterday

## Claude responded: Here's a 2D pseudo-spectral solver.

Writing the FFT, diagnostics, and solver code in Rust.

Writing the FFT, diagnostics, and solver code in Rust.

Here’s a 2D pseudo-spectral solver. In vorticity-stream-function form, divergence-free flow is built in and no pressure solve is needed. It also includes a built-in validation case.

Ran 4 commands, created 2 files

Ran 4 commands, created 2 files

Build scripts can't execute from the outputs mount, so I'll build in a scratch directory instead.

The decay case is off by 2.5%, and the cause is a bug in my loop. On the final iteration it breaks before taking the last step, so the run stops at t = 0.99 instead of 1.0. I’ll fix that.

Edited a file, ran a command, shared files

Edited a file, ran a command, shared files

I built the solver and ran it. `ns2d` is a pseudo-spectral 2D Navier-Stokes solver in vorticity form, with 2/3 dealiasing, integrating-factor RK4 and CFL-adaptive dt, using `rustfft` as the only dependency.

Run it with `cargo run --release -- 64 0.01 1.0 decay`. The arguments are `n`, `nu`, `t_end` and the initial condition (`decay` or `mixed`).

**Validation:**

- **Decay case:** Your initial condition, rewritten as a stream function so it’s divergence-free, is a single Laplacian eigenmode, so the nonlinear term vanishes exactly. Enstrophy should decay as exp(−2ν·125·t), and the solver matches that to a relative error of 3.6e-15.
- **Mixed case:** This adds modes of different |k| so the advection term is active. At t = 1, enstrophy is 0.2549 of its initial value at n = 128 versus 0.2566 at n = 64. That 0.7% difference is the resolution error.
- **Tail diagnostic:** The `tail` column is the fraction of enstrophy in the top quarter of retained wavenumbers. It’s an under-resolution alarm, and it rises to about 12% in the mixed case at n = 64. Treat any run where it is large with suspicion.

**Two things to know:**

- **Initial condition:** Your v field isn’t exactly reproducible. A stream function that gives your u exactly makes v = −2cos(10x)sin(5y). The comparison with your original code is therefore only qualitative.
- **Bug I fixed:** My first version stopped one step early, which made the validation error 2.5%. I fixed it before delivering.

**Extending to 3D:**

- **Variables:** Swap the vorticity-stream-function form for primitive variables û(k) with Leray projection, û ← û − k(k·û)/|k|², applied after each nonlinear evaluation.
- **FFTs:** 3D transforms are three passes of the same row-FFT-plus-transpose pattern.
- **Diagnostics:** Track max |ω|, enstrophy and the spectral tail as above, run a resolution ladder, and look for a power-law divergence that persists as you refine.

I can write the 3D version next, with Taylor-Green vortex as the validation case, since it has well-documented reference enstrophy curves.



Main

RS 

Download



Cargo

TOML 

Download

Download all

yesterday

## You said: yes

yes

yesterday

## Claude responded: The search confirms the dissipation peak at t ≈ 9 for Re = 1600, but it doesn't give the peak magnitude, so I'll only claim what's verified.

Searched the web

Searched the web

The search confirms the dissipation peak at t ≈ 9 for Re = 1600, but it doesn’t give the peak magnitude, so I’ll only claim what’s verified. Now the 3D solver.

Ran 9 commands, created 2 files

Ran 9 commands, created 2 files

At about 6 s per step on this single core, the t = 9 peak is roughly an hour away. That’s slower than it should be, so I’ll profile before deciding what to do.

Ran 2 commands

Ran 2 commands

The solver costs about 0.55 s per step, so the earlier background run was starved while the sandbox was idle. A foreground run to t = 12 should take roughly four minutes.

Ran 5 commands, shared files

Ran 5 commands, shared files

The 3D solver is written and built, and it passed the validation runs I could finish. The Taylor-Green run at n = 64 only reached t = 8.75, because the sandbox has one core and stalls between commands.

**How it works**

- **Method:** It integrates û directly in rotational form, with the pressure removed by Leray projection. It uses 2/3 dealiasing, integrating-factor RK4 and CFL-adaptive dt.
- **FFTs and parallelism:** The 3D FFT is built from `rustfft` row transforms and parallelized with `rayon`.
- **Run it:** `cargo run --release -- 128 1600 20 tg`, with arguments `n`, `Re`, `t_end` and `ic` (`tg` or `abc`).
- **Output:** It writes a CSV of t, E, Z, ε, max|ω| and the running ∫max|ω|dt, which is the Beale-Kato-Majda quantity from earlier.

**What I checked**

- **ABC (Beltrami) flow, n = 32, Re = 20:** The exact solution is pure decay, and the energy ratio matched exp(−2νt) to a relative error of 2e-15. Divergence stayed at 3e-28.
- **Near-inviscid Taylor-Green, n = 32:** Energy was conserved to six digits over t = 3 while enstrophy grew 2.4×. This is the check that exercises the nonlinear term.
- **Taylor-Green initial values:** E₀ = 0.125 and Z₀ = 0.375, as exact.
- **Taylor-Green at Re = 1600, n = 64:** The dissipation rate ε = 2νZ climbed from 4.7e-4 at t = 0 to 0.0131 at t = 8.75, still rising. The benchmark literature puts the dissipation peak at t ≈ 9, which I confirmed by search. I didn’t verify the literature’s peak value, so I won’t claim agreement on magnitude.

**What the n = 64 run can’t tell you**

- **Under-resolution:** The `tail` column, the fraction of enstrophy in the top quarter of retained wavenumbers, hit 37% by t = 6. Re = 1600 needs n = 128 or higher near the peak, so treat my n = 64 numbers as qualitative.
- **Single-run caveat:** I never saw the peak itself or the decay after it.

**For your tests**

- Run n = 128 and n = 256 on a multicore machine and compare ε(t) across resolutions. Then check whether max|ω| and the BKM integral stay bounded as resolution increases.
- For a blowup candidate, replace the `tg` branch in `initial()` with the construction’s exact data.
- On my sandbox’s old rustc I had to pin `rayon`/`rayon-core` to older versions. You won’t need that on a current toolchain.



Main

RS 

Download



Cargo

TOML 

Download

Download all

yesterday



is









[Claude is AI and can make mistakes. Please double-check cited sources.](https://support.anthropic.com/en/articles/8525154-claude-is-providing-incorrect-or-misleading-responses-what-s-going-on)

Sonnet 5.5 Medium

<iframe height="1" width="1" style="position: absolute; top: 0px; left: 0px; border-width: medium; border-style: none; border-color: currentcolor; border-image: none; visibility: hidden;"></iframe>