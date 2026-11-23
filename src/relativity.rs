/// Albert Einstein & Relativity Station (1905/1915)
///
/// Demonstrates special and general relativity through concrete examples:
/// time dilation (Hafele-Keating airplane clocks), proper time, length contraction,
/// relativistic Doppler shift, gravitational redshift, and the twin paradox.
/// Shows why relativity and quantum mechanics resist unification.
///
/// Historical context:
/// - Born 1879 in Ulm, Germany; died 1955 in Princeton, New Jersey
/// - Special relativity (1905): "On the Electrodynamics of Moving Bodies"
/// - General relativity (1915): field equations describing gravity as spacetime curvature
/// - Hafele-Keating experiment (1971): atomic clocks on airplanes confirmed time dilation
/// - 1921 Nobel Prize in Physics (photoelectric effect, not relativity — committee was cautious)

/// Station metadata for versioning and formalization tracking.
pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "relativity",
    version: "0.0.1",
    formalized: false,
    notes: "Special and general relativity: time dilation, proper time, Doppler shift",
};

use std::f64::consts::PI;

pub fn show_relativity_station() {
    println!("=== ALBERT EINSTEIN & RELATIVITY (1905/1915) ===");
    println!();
    print_biography();
    println!();
    explain_special_relativity();
    println!();
    demonstrate_time_dilation();
    println!();
    demonstrate_hafele_keating();
    println!();
    explain_proper_time();
    println!();
    demonstrate_length_contraction();
    println!();
    demonstrate_relativistic_doppler();
    println!();
    demonstrate_gravitational_redshift();
    println!();
    demonstrate_twin_paradox();
    println!();
    explain_relativity_vs_quantum();
}

fn print_biography() {
    println!("Albert Einstein (1879-1955)");
    println!("German-born theoretical physicist, patent clerk turned revolutionary");
    println!();
    println!("In 1905 ('annus mirabilis'), a 26-year-old patent clerk published four papers.");
    println!("One of them, 'On the Electrodynamics of Moving Bodies,' showed that space and");
    println!("time are not absolute. They depend on your motion. Ten years later, he extended");
    println!("this to gravity itself: mass curves spacetime, and what we feel as gravity is");
    println!("objects following curved paths through that geometry.");
}

fn explain_special_relativity() {
    println!("--- SPECIAL RELATIVITY (1905) ---");
    println!();
    println!("Two postulates, from which everything follows:");
    println!();
    println!("  1. The laws of physics are the same in all inertial reference frames.");
    println!("     No experiment can tell you if you're 'really' moving or at rest.");
    println!();
    println!("  2. The speed of light (c) is the same for ALL observers, regardless of");
    println!("     their motion or the motion of the source. c = 299,792,458 m/s, always.");
    println!();
    println!("Consequence: If light speed is fixed and you're moving toward a light source,");
    println!("you don't measure faster light. Instead, YOUR time slows down and YOUR space");
    println!("contracts. Space and time are the flexible parts; c is the constant.");
}

fn demonstrate_time_dilation() {
    println!("--- TIME DILATION: Moving clocks run slow ---");
    println!();
    println!("Formula: Δt_moving = Δt_stationary / sqrt(1 - v²/c²)");
    println!("         where γ (gamma) = 1 / sqrt(1 - v²/c²) is the Lorentz factor");
    println!();

    let c = 299_792_458.0_f64; // speed of light m/s
    let velocities = [
        (300.0, "Commercial airplane"),
        (7_670.0, "ISS orbital velocity"),
        (29_979_245.8, "10% of c"),
        (269_813_212.2, "90% of c"),
        (296_794_533.42, "99% of c"),
    ];

    println!("{:<28} {:>12} {:>12} {:>12}", "Scenario", "v (m/s)", "γ factor", "1h on clock");
    println!("{}", "-".repeat(70));

    for &(v, label) in &velocities {
        let beta = v / c;
        let gamma = 1.0 / (1.0 - beta * beta).sqrt();
        let one_hour_stationary = 3600.0; // seconds
        let one_hour_moving = one_hour_stationary * gamma;
        let diff_ms = (one_hour_moving - one_hour_stationary) * 1000.0;

        println!(
            "{:<28} {:>12.1} {:>12.6} {:>7.3}s (+{:.4}ms)",
            label, v, gamma, one_hour_moving, diff_ms
        );
    }

    println!();
    println!("At airplane speeds, the effect is tiny (nanoseconds per hour).");
    println!("But it's REAL — and we've measured it. See Hafele-Keating below.");
}

fn demonstrate_hafele_keating() {
    println!("--- HAFELE-KEATING EXPERIMENT (1971) ---");
    println!();
    println!("Joseph Hafele and Richard Keating put cesium atomic clocks on commercial");
    println!("airlines, flew them around the world in both directions, and compared them");
    println!("to reference clocks at the U.S. Naval Observatory.");
    println!();
    println!("Two effects compete:");
    println!("  • Special relativity (velocity): moving clock runs SLOWER");
    println!("  • General relativity (altitude): higher clock runs FASTER (less gravity)");
    println!();

    // Approximate values from the actual experiment
    let eastbound_predicted = -40.0;  // nanoseconds (net loss)
    let eastbound_measured = -59.0;   // ±10 ns uncertainty
    let westbound_predicted = 275.0;  // nanoseconds (net gain)
    let westbound_measured = 273.0;   // ±10 ns uncertainty

    println!("{:<20} {:>14} {:>14}", "Direction", "Predicted (ns)", "Measured (ns)");
    println!("{}", "-".repeat(50));
    println!(
        "{:<20} {:>14.0} {:>14.0} ± 10",
        "Eastbound", eastbound_predicted, eastbound_measured
    );
    println!(
        "{:<20} {:>14.0} {:>14.0} ± 10",
        "Westbound", westbound_predicted, westbound_measured
    );

    println!();
    println!("The measured values agreed with predictions within experimental uncertainty.");
    println!("Time dilation is not theoretical. Your GPS satellites correct for it");
    println!("every day — without the correction, your navigation would drift by");
    println!("about 10 kilometers per day.");
}

fn explain_proper_time() {
    println!("--- PROPER TIME: The time along YOUR path ---");
    println!();
    println!("Every object carries its own clock through spacetime. The time that clock");
    println!("measures is called PROPER TIME (τ). It's the time experienced by the object.");
    println!();
    println!("Key insight: Two objects can start together, take different paths through");
    println!("spacetime, and reunite with DIFFERENT amounts of proper time elapsed.");
    println!();
    println!("This is not a measurement error. The traveling clock has genuinely experienced");
    println!("less time. From its own perspective, nothing was slow — it just took a shorter");
    println!("path through spacetime (in the 4D sense).");
    println!();
    println!("A photon follows a NULL path: its proper time is exactly ZERO.");
    println!("There is no such thing as 'from the photon's perspective' in any meaningful");
    println!("sense. For light, emission and absorption are the same event.");
}

fn demonstrate_length_contraction() {
    println!("--- LENGTH CONTRACTION: Moving objects shrink ---");
    println!();
    println!("Formula: L_moving = L_stationary × sqrt(1 - v²/c²) = L_stationary / γ");
    println!();
    println!("An object moving relative to you appears shorter along its direction of motion.");
    println!("Not an optical illusion — a physical consequence of how spacetime works.");
    println!();

    let c = 299_792_458.0_f64;
    let proper_length = 100.0; // meters at rest
    let velocities = [
        (29_979_245.8, "10% of c"),
        (269_813_212.2, "90% of c"),
        (296_794_533.42, "99% of c"),
    ];

    println!("{:<28} {:>12} {:>12}", "Scenario", "γ factor", "Observed length");
    println!("{}", "-".repeat(55));

    for &(v, label) in &velocities {
        let beta = v / c;
        let gamma = 1.0 / (1.0 - beta * beta).sqrt();
        let observed = proper_length / gamma;
        println!(
            "{:<28} {:>12.6} {:>10.4} m",
            label, gamma, observed
        );
    }

    println!();
    println!("At 99% of c, a 100m spaceship appears to be only 14.1m long to you.");
    println!("But to the astronauts inside, YOU are the one who's squished.");
}

fn demonstrate_relativistic_doppler() {
    println!("--- RELATIVISTIC DOPPLER SHIFT: Blue shift & Red shift ---");
    println!();
    println!("When a light source moves relative to you, its frequency changes:");
    println!("  • Approaching → higher frequency → BLUE SHIFT (light looks bluer)");
    println!("  • Receding   → lower frequency  → RED SHIFT  (light looks redder)");
    println!();
    println!("Formula (longitudinal): f_observed = f_source × sqrt((1+β)/(1-β))");
    println!("                          where β = v/c (positive when approaching)");
    println!();

    let c = 299_792_458.0_f64;
    let source_freq = 500.0e12; // 500 THz (green light, ~600nm)
    let velocities = [
        (29_979_245.8, "Approaching at 10% c"),
        (-29_979_245.8, "Receding at 10% c"),
        (269_813_212.2, "Approaching at 90% c"),
        (-269_813_212.2, "Receding at 90% c"),
    ];

    println!("{:<28} {:>14} {:>14}", "Scenario", "Observed freq", "Shift");
    println!("{}", "-".repeat(60));

    for &(v, label) in &velocities {
        let beta = v / c;
        let doppler_factor = ((1.0 + beta) / (1.0 - beta)).sqrt();
        let observed_freq = source_freq * doppler_factor;
        let shift_pct = (doppler_factor - 1.0) * 100.0;

        let color_hint = if observed_freq > source_freq {
            "← BLUER"
        } else {
            "REDDER →"
        };

        println!(
            "{:<28} {:>12.1} THz {:>+10.1}% {}",
            label,
            observed_freq / 1e12,
            shift_pct,
            color_hint
        );
    }

    println!();
    println!("This is why astronomers can tell galaxies are moving away from us:");
    println!("their light is redshifted. The farther a galaxy is, the faster it recedes.");
    println!("That's how we know the universe is expanding.");
}

fn demonstrate_gravitational_redshift() {
    println!("--- GRAVITATIONAL REDSHIFT (General Relativity) ---");
    println!();
    println!("Light climbing OUT of a gravitational well loses energy → redshifts.");
    println!("Light falling IN gains energy → blueshifts.");
    println!();
    println!("Formula (weak field): Δf/f ≈ -ΔΦ/c²  where ΔΦ is the potential difference");
    println!();

    // Earth's surface to infinity approximation
    let g = 9.81;        // m/s²
    let r_earth = 6.371e6; // meters
    let c = 299_792_458.0_f64;

    // Gravitational potential at surface: Φ = -GM/r = -g*r (approximation)
    let phi_surface = -g * r_earth;
    let shift_fraction = phi_surface / (c * c);

    println!("Example: Light from Earth's surface to far space");
    println!("  Gravitational potential at surface: {:.2e} J/kg", phi_surface);
    println!(
        "  Fractional frequency shift: Δf/f ≈ {:.3e}",
        shift_fraction.abs()
    );
    println!();
    println!("  For green light (500 THz):");
    let green_freq = 500.0e12;
    let shifted = green_freq * (1.0 + shift_fraction);
    println!(
        "    Shifted frequency: {:.6} THz ({:.3e} Hz lower)",
        shifted / 1e12,
        (green_freq - shifted)
    );
    println!();
    println!("Tiny on Earth, but enormous near black holes. Light escaping a");
    println!("black hole's event horizon would be infinitely redshifted — it never");
    println!("gets out. That's what the 'event horizon' actually means.");
}

fn demonstrate_twin_paradox() {
    println!("--- THE TWIN PARADOX ---");
    println!();
    println!("Twin A stays on Earth. Twin B flies to a star at 90% of c and returns.");
    println!("Who is older when they reunite?");
    println!();
    println!("It seems paradoxical: from B's perspective, A is the one moving, so");
    println!("A should be younger. But the situation isn't symmetric — B accelerates");
    println!("(turns around), while A doesn't. The twin who changes frames ages less.");
    println!();

    let c = 299_792_458.0_f64;
    let v = 0.9 * c; // 90% of c
    let distance_ly = 4.0; // light-years to the star (one way)
    let gamma = 1.0_f64 / (1.0_f64 - 0.81_f64).sqrt();

    // Time for round trip in Earth's frame
    let earth_time_years = 2.0 * distance_ly / 0.9; // 2 × 4ly / 0.9c
    // Time experienced by traveling twin
    let traveler_time_years = earth_time_years / gamma;

    println!("Parameters:");
    println!("  Velocity: 0.9c (γ = {:.4})", gamma);
    println!("  Distance to star: {} light-years (one way)", distance_ly);
    println!();
    println!("Earth frame calculation:");
    println!("  Round trip time on Earth: {:.2} years", earth_time_years);
    println!(
        "  Time experienced by traveler: {:.2} years ({:.2} / γ)",
        traveler_time_years, earth_time_years
    );
    println!();
    println!("Result: When they reunite:");
    println!("  Twin A (Earth):    aged {:.2} years", earth_time_years);
    println!("  Twin B (traveler): aged {:.2} years", traveler_time_years);
    println!(
        "  Difference:           {:.2} years",
        earth_time_years - traveler_time_years
    );
    println!();
    println!("Not a paradox. The traveling twin genuinely experienced less time.");
    println!("This has been verified with atomic clocks on airplanes and satellites.");
}

fn explain_relativity_vs_quantum() {
    println!("--- WHY RELATIVITY AND QUANTUM MECHANICS DON'T MIX ---");
    println!();
    println!("Special relativity says: no influence can travel faster than c.");
    println!("Quantum entanglement says: measuring particle A instantaneously determines");
    println!("the state of particle B, regardless of distance.");
    println!();
    println!("This looks like a contradiction — but it's not quite. Entanglement doesn't");
    println!("transmit information faster than light (you can't control the outcome).");
    println!("Still, the tension is real:");
    println!();
    println!("  • Relativity: spacetime is smooth, deterministic geometry.");
    println!("  • Quantum: reality is probabilistic, discrete, and measurement-dependent.");
    println!();
    println!("We have a theory for very fast (relativity) and a theory for very small");
    println!("(quantum), but no complete theory that handles both at once. That's the");
    println!("holy grail: quantum gravity. String theory, loop quantum gravity, and");
    println!("other approaches are all attempts to reconcile these two pillars.");
    println!();
    println!("CONNECTION TO THE GYM: The stations below (Bell, Wheeler, von Neumann,");
    println!("Everett, Rovelli) all grapple with what quantum measurement means — and");
    println!("why it resists the clean, observer-independent descriptions that relativity");
    println!("gives us for spacetime.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorentz_factor_at_rest() {
        let c = 299_792_458.0_f64;
        let v = 0.0;
        let gamma = 1.0 / (1.0 - (v / c).powi(2)).sqrt();
        assert!((gamma - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_lorentz_factor_at_90c() {
        let c = 299_792_458.0_f64;
        let v = 0.9 * c;
        let gamma = 1.0 / (1.0 - (v / c).powi(2)).sqrt();
        // γ at 0.9c = 1/sqrt(1-0.81) = 1/sqrt(0.19) ≈ 2.294
        assert!((gamma - 2.294157).abs() < 1e-4);
    }

    #[test]
    fn test_time_dilation_moving_clock_slower() {
        let c = 299_792_458.0_f64;
        let v = 0.9 * c;
        let gamma = 1.0 / (1.0 - (v / c).powi(2)).sqrt();
        let stationary_time = 3600.0;
        let moving_time = stationary_time * gamma;
        assert!(moving_time > stationary_time);
    }

    #[test]
    fn test_length_contraction_moving_object_shorter() {
        let c = 299_792_458.0_f64;
        let v = 0.9 * c;
        let gamma = 1.0 / (1.0 - (v / c).powi(2)).sqrt();
        let proper_length = 100.0;
        let observed_length = proper_length / gamma;
        assert!(observed_length < proper_length);
        assert!((observed_length - 43.589).abs() < 0.01);
    }

    #[test]
    fn test_relativistic_doppler_approaching_blueshift() {
        let c = 299_792_458.0_f64;
        let v = 0.1 * c;
        let beta = v / c;
        let factor = ((1.0 + beta) / (1.0 - beta)).sqrt();
        assert!(factor > 1.0); // approaching → higher frequency
    }

    #[test]
    fn test_relativistic_doppler_receding_redshift() {
        let c = 299_792_458.0_f64;
        let v = -0.1 * c;
        let beta = v / c;
        let factor = ((1.0 + beta) / (1.0 - beta)).sqrt();
        assert!(factor < 1.0); // receding → lower frequency
    }

    #[test]
    fn test_twin_paradox_traveler_ages_less() {
        let c = 299_792_458.0_f64;
        let v = 0.9 * c;
        let gamma = 1.0 / (1.0 - (v / c).powi(2)).sqrt();
        let earth_time = 2.0 * 4.0 / 0.9;
        let traveler_time = earth_time / gamma;
        assert!(traveler_time < earth_time);
    }

    #[test]
    fn test_gravitational_redshift_negative() {
        let g = 9.81;
        let r_earth = 6.371e6;
        let c = 299_792_458.0_f64;
        let phi_surface = -g * r_earth;
        let shift_fraction = phi_surface / (c * c);
        assert!(shift_fraction < 0.0); // light climbing out loses frequency
    }

    #[test]
    fn test_hafele_keating_within_uncertainty() {
        let eastbound_predicted = -40.0;
        let eastbound_measured = -59.0;
        let westbound_predicted = 275.0;
        let westbound_measured = 273.0;
        let uncertainty = 10.0;

        // Both within ~2σ of prediction
        assert!(((eastbound_measured - eastbound_predicted) as f64).abs() < 3.0_f64 * (uncertainty as f64));
        assert!(((westbound_measured - westbound_predicted) as f64).abs() < 3.0_f64 * (uncertainty as f64));
    }
}
