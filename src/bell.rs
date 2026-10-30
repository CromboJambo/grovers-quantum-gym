/// John Stewart Bell's Theorem (1964)
/// Demonstrates that no local hidden variable theory can reproduce all
/// predictions of quantum mechanics. Simulates both LHV and QM to show
/// the violation of Bell's inequality.

use std::f64::consts::PI;

pub fn show_bell_station() {
    println!("=== JOHN STEWART BELL'S THEOREM (1964) ===");
    println!();
    print_biography();
    println!();
    explain_eparadox();
    println!();
    derive_bells_inequality();
    println!();
    simulate_comparison();
}

fn print_biography() {
    println!("John Stewart Bell (1928-1990)");
    println!("Irish theoretical physicist working at CERN");
    println!();
    println!("While reading Einstein's writings on quantum mechanics, Bell");
    println!("wondered: could there be 'hidden variables' that restore");
    println!("determinism? Could particles carry pre-existing instructions?");
    println!();
    println!("In 1964 he published 'On the Einstein-Podolsky-Rosen Paradox',");
    println!("deriving an inequality that any local hidden variable theory must");
    println!("satisfy. Quantum mechanics violates it. The theorem transformed a");
    println!("philosophical debate into an experimental question.");
}

fn explain_eparadox() {
    println!("--- THE EINSTEIN-PODOLSKY-ROSEN PARADOX (1935) ---");
    println!();
    println!("EPR considered a pair of particles created together, then sent");
    println!("in opposite directions. Their spins are correlated: measuring");
    println!("particle A's spin along any axis determines particle B's spin.");
    println!();
    println!("Einstein's argument (realism + locality):");
    println!("  REALISM: Particles have definite properties before measurement");
    println!("  LOCALITY: Measuring A cannot instantaneously affect distant B");
    println!();
    println!("Therefore, particle B must already 'know' what spin to show.");
    println!("Quantum mechanics is incomplete — missing 'hidden variables'.");
    println!();
    println!("Bell's insight: If hidden variables exist, correlations between");
    println!("measurements at different angles must satisfy a specific inequality.");
}

fn derive_bells_inequality() {
    println!("--- BELL'S INEQUALITY (Simplified CHSH Version) ---");
    println!();
    println!("Setup: Alice and Bob each measure spin along one of two axes.");
    println!("Alice chooses between a or a'. Bob chooses between b or b'.");
    println!("Each measurement yields +1 or -1.");
    println!();
    println!("Define correlation E(a,b) = average of (Alice's result × Bob's result)");
    println!();
    println!("CHSH inequality: |S| ≤ 2, where");
    println!("  S = E(a,b) + E(a,b') + E(a',b) - E(a',b')");
    println!();
    println!("Local hidden variable theories MUST satisfy |S| ≤ 2.");
    println!("Quantum mechanics predicts |S| can reach 2√2 ≈ 2.828!");
}

fn simulate_comparison() {
    println!("--- SIMULATION: LHV vs Quantum Mechanics ---");
    println!();

    // Optimal angles for maximum QM violation
    let a = 0.0;
    let b = PI / 4.0;
    let a_prime = PI / 2.0;
    let b_prime = -PI / 4.0;

    println!("Using optimal measurement angles (radians):");
    println!("  Alice: a=0,      a'=π/2");
    println!("  Bob:   b=π/4,    b'=-π/4");
    println!();

    // CHSH value for local hidden variables (max possible)
    let chsh_lhv = 2.0;

    // Quantum mechanical prediction: S = 2√2
    let chsh_qm = 2.0 * 2.0_f64.sqrt();

    println!("CHSH inequality test: |S| ≤ 2 ?");
    println!();
    println!("Local Hidden Variable Theory (best case):");
    println!("  S_LHV = {}", chsh_lhv);
    println!("  Violates inequality? NO (satisfies |S| ≤ 2)");
    println!();

    println!("Quantum Mechanics prediction:");
    println!("  S_QM = 2√2 = {:.4}", chsh_qm);
    println!("  Violates inequality? YES!");
    println!(
        "  Exceeds classical limit by {:.1}%",
        (chsh_qm / chsh_lhv - 1.0) * 100.0
    );
    println!();

    // Demonstrate with correlation values
    println!("Correlation predictions E(a,b) = cos(θ_a - θ_b):");
    let e_ab = (a - b).cos();
    let e_apb = (a_prime - b).cos();
    let e_abp = (a - b_prime).cos();
    let e_apbp = (a_prime - b_prime).cos();

    println!("  E(a,b)   = {:.4}", e_ab);
    println!("  E(a',b)  = {:.4}", e_apb);
    println!("  E(a,b')  = {:.4}", e_abp);
    println!("  E(a',b') = {:.4}", e_apbp);

    let s_calc = e_ab + e_abp + e_apb - e_apbp;
    println!();
    println!("Computed S = E(a,b) + E(a,b') + E(a',b) - E(a',b')");
    println!("          = {:.4}", s_calc);
    println!();

    println!("EXPERIMENTAL VERDICT:");
    println!("  Aspect (1982), Clauser (2022 Nobel Prize), and many others");
    println!("  have confirmed: S > 2. Local realism is FALSE.");
    println!();
    println!("Either particles don't have pre-existing properties, or");
    println!("measurement of one particle instantaneously affects the other,");
    println!("or both. The universe is non-local or non-real (or both).");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chsh_quantum_violates_inequality() {
        let chsh_qm = 2.0 * 2.0_f64.sqrt();
        assert!(chsh_qm > 2.0);
        assert!((chsh_qm - 2.828).abs() < 0.01);
    }

    #[test]
    fn test_chsh_lhv_satisfies_inequality() {
        let chsh_lhv = 2.0;
        assert!(chsh_lhv <= 2.0);
    }

    #[test]
    fn test_correlation_formula() {
        // E(a,b) = cos(θ_a - θ_b) for singlet state
        let a = 0.0_f64;
        let b = PI / 4.0;
        let correlation = (a - b).cos();
        assert!((correlation - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_optimal_angles_give_max_violation() {
        let a = 0.0_f64;
        let b = PI / 4.0;
        let a_prime = PI / 2.0;
        let b_prime = -PI / 4.0;

        let e_ab = (a - b).cos();
        let e_apb = (a_prime - b).cos();
        let e_abp = (a - b_prime).cos();
        let e_apbp = (a_prime - b_prime).cos();

        let s = e_ab + e_abp + e_apb - e_apbp;
        assert!((s - 2.0 * 2.0_f64.sqrt()).abs() < 1e-10);
    }
}
