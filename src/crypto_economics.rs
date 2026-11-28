//! Cryptographic Economics Station (November 2026)
//!
//! The negative value of security through obscurity in the post-P world.
//!
//! Context: In late November 2026, a cluster of AI-generated mathematical
//! results forced a re-examination of the economic assumptions underpinning
//! public-key cryptography:
//!   - VP ≠ VNP over every field (OpenAI Nov 23 & Anthropic/Claude Nov 23)
//!   - Matrix multiplication exponent ω ≤ 9/4 (OpenAI, Nov 24)
//!   - Unconditional L_N[1/3, O(1)] integer factorization (Google Cogentic/Gemini 4, Nov 25)
//!   - Unverified rumors of polynomial-time factoring at AI labs (Aaronson, Nov 24-26)
//!
//! This station covers:
//!   1. What is real vs. rumored (as of Nov 27, 2026)
//!   2. Subexponential ≠ polynomial: why L_N[1/3] does not break RSA today
//!   3. RSA (factoring) vs. Bitcoin/ECC (discrete log): different problems
//!   4. The cost of obscurity layers vs. actual security controls
//!   5. What survives an impairment event: access control, rotation, audit
//!
//! Primary sources:
//!   - Aaronson, S. "Shtetl-Optimized" blog comments & post p=10379 (Nov 24-26 2026)
//!     https://scottaaronson.blog/?p=10379
//!   - Castelvecchi, D. "Has AI just cracked Internet encryption?" Nature (Nov 26 2026)
//!     https://www.nature.com/articles/d41586-026-03875-6
//!   - Google Research / Cogentic: "Unconditional Integer Factorization in Time L_N[1/3, O(1)]"
//!     github.com/google-research/google-research/blob/master/cogentic_research_papers/integer_factorization/README.md
//!   - OpenAI: "VP and VNP are different" (Nov 23) & "matrix multiplication exponent is nine-fourths" (Nov 24)
//!     github.com/openai/math/tree/main/preprints
//!   - Anthropic / Claude: "VP ≠ VNP over every field" (Nov 23), PR #51 anthropics/formal-math
//!     Local copy: papers/vp_vnp/
//!
//! Central lesson: cryptographic assumptions are long-lived intangible assets.
//! When the underlying hardness assumption is challenged, the asset faces an
//! impairment test. Obscurity layers have zero residual value in that scenario.

pub const STATION: crate::StationMeta = crate::StationMeta {
    name: "crypto_economics",
    version: "0.0.1",
    formalized: false,
    notes: "Cryptographic assumption economics: L_N[1/3] vs poly, RSA vs ECC, cost of obscurity (Nov 2026 AI math cluster)",
};

/// Compute the L_x(α, c) function from subexponential complexity analysis.
/// L_x(α, c) = exp(c * (ln x)^α * (ln ln x)^(1-α))
/// α=0: exponential; α=1: polynomial; 0<α<1: subexponential.
fn l_function(x: f64, alpha: f64, c: f64) -> f64 {
    let ln_x = x.ln();
    let ln_ln_x = ln_x.ln();
    (c * ln_x.powf(alpha) * ln_ln_x.powf(1.0 - alpha)).exp()
}

/// Why L_N[1/3] (Google/Cogentic) does not break RSA-2048 today.
fn demonstrate_complexity_gap() {
    println!("\n--- EXPERIMENT 1: THE COMPLEXITY GAP ---");
    println!("RSA-2048 modulus N has ~617 decimal digits, ln(N) ≈ 1418.\n");
    println!("{:>18} | {:>16} | {}", "Algorithm", "Operations (est.)", "RSA-2048 impact");
    println!("{}", "-".repeat(65));

    let ln_n = 2048.0 * 2.0_f64.ln(); // ≈ 1418

    // Polynomial: N^1 — would mean factoring is easy
    let poly = l_function(ln_n, 1.0, 1.0);
    println!("{:>18} | {:>16.0e} | instant — RSA broken", "N^1 (polynomial)", poly);

    // Google/Cogentic claimed bound: L_N[1/3, 105]
    let cogentic = l_function(ln_n, 1.0/3.0, 105.0);
    println!("{:>18} | {:>16.0e} | theoretical, not practical", "L_N[1/3, 105]", cogentic);

    // Current best classical: GNFS = L_N[1/3, (64/9)^(1/3)] ≈ L_N[1/3, 1.923]
    let gnfs = l_function(ln_n, 1.0/3.0, f64::cbrt(64.0 / 9.0));
    println!("{:>18} | {:>16.0e} | current best, RSA safe", "GNFS L_N[1/3,1.92]", gnfs);

    // Exponential (obsolete algorithms)
    let exp_old = 2.0_f64.powf((ln_n * ln_n.ln()).sqrt());
    println!("{:>18} | {:>16.0e} | obsolete", "2^√(lnN·lnlnN)", exp_old);

    println!("\nKEY: Cogentic's L_N[1/3, 105] has the SAME α=1/3 as GNFS.");
    println!("Its constant c=105 is WORSE than GNFS's c≈1.92 at this size.");
    println!("This is a THEORETICAL result (unconditional, zero-error), not an attack.");
}

/// All "custom" secret formats collapse to the same regex shapes.
fn demonstrate_obscurity_theater() {
    println!("\n--- EXPERIMENT 2: THE ILLUSION OF CUSTOMIZATION ---");
    println!("Different naming conventions, same parser output.\n");

    // (input line, human label)
    let samples = vec![
        ("DB_PWD=Sup3rS3cret!2024", "mangled secret"),
        ("AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE", "AWS key ID"),
        ("OPENAI_API_KEY=sk-abc123def456ghi789jkl012", "vendor-prefixed token"),
        ("my_db_pw = \"hunter2_modified\"", "custom naming"),
        ("auth_credential_token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...", "JWT blob"),
    ];

    for (line, label) in &samples {
        println!("  {:<55} -> [{}]", line, label);
    }

    println!("\nCONCLUSION: A regex parser (or AI scanner) sees four shapes:");
    println!("  1. Base64-ish blobs (JWTs, encoded keys)");
    println!("  2. Hex strings (hashes, raw key material)");
    println!("  3. Vendor-prefixed identifiers (sk-, AKIA, ghp_, xoxb-)");
    println!("  4. Human-mangled secrets (Sup3rS3cret!)");
    println!();
    println!("Custom naming adds maintenance cost and zero security.");
}

/// RSA security depends on factoring; Bitcoin/ECC depends on discrete log.
fn demonstrate_rsa_vs_ecc() {
    println!("\n--- EXPERIMENT 3: RSA vs ECC — DIFFERENT PROBLEMS ---");
    println!("A breakthrough in one does NOT automatically break the other.\n");

    // RSA: security from factoring N = p*q
    let p = 61u64;
    let q = 53u64;
    let n = p * q;
    println!("RSA: given N = {} (={p}x{q}), find p and q.", n);
    println!("     Problem: INTEGER FACTORIZATION");
    println!();

    // ECC: security from discrete log on elliptic curve
    let ec_p = 23u64; // small prime for demo
    let a = 1u64;
    let b = 1u64;
    println!("ECC (Bitcoin secp256k1): given curve y^2=x^3+{a}x+{b} mod {ec_p},");
    println!("     generator G, and point P = k*G, find scalar k.");
    println!("     Problem: ELLIPTIC CURVE DISCRETE LOGARITHM");
    println!();

    // Show that factoring n doesn't help with discrete log
    println!("Factoring N={n} gives us p={p}, q={q}. Does that help find k?");
    println!("NO. Different mathematical structures entirely.");
    println!();
    println!("Bitcoin uses ECDSA/Schnorr on secp256k1 (BIP-340), NOT RSA.");
    println!("RSA break != Bitcoin break. But ECDLP break = Bitcoin crisis.");
    println!();
    println!("Aaronson's rumors span BOTH fronts: factoring AND elliptic curves.");
}

/// The impairment test: what happens to security line items when the
/// cryptographic assumption they depend on is challenged?
fn demonstrate_impairment_test() {
    println!("\n--- EXPERIMENT 4: THE IMPAIRMENT TEST ---");
    println!("When the math breaks, which controls survive?\n");

    // (control, cost_per_year, depends_on_hardness, survives)
    let controls = vec![
        ("Custom env file naming", 4200, false, true),
        ("Base64-encoding secrets", 1800, false, true),
        ("Three tunnel configurations", 12000, false, true),
        ("Secrets manager + rotation", 3500, true, true),
        ("RSA-2048 TLS certificates", 1000, true, false),
        ("ECC-based signatures (Bitcoin)", 0, true, false),
    ];

    println!("{:<35} | {:>10} | {:>12} | {}", "Control", "Cost/yr", "Depends on", "Survives P=NP?");
    println!("{}", "-".repeat(75));

    for (name, cost, depends, survives) in &controls {
        let dep = if *depends { "hardness" } else { "process" };
        let surv = if *survives { "YES" } else { "NO - impaired" };
        println!("{:<35} | ${:>9} | {:>12} | {}", name, cost, dep, surv);
    }

    println!();
    println!("OBSURITY LAYERS (process-based):");
    println!("  Cost money, create friction, fail silently.");
    println!("  Survive the math breaking — but they were never security anyway.");
    println!();
    println!("HARDNESS-BASED CONTROLS:");
    println!("  RSA/ECC: face immediate impairment if factoring/ECDLP becomes poly-time.");
    println!("  Rotation + access control: survive because they don't assume the math holds forever.");
}

/// The main station entry point.
pub fn show_crypto_economics_station() {
    println!("=== CRYPTOGRAPHIC ECONOMICS STATION (Nov 2026) ===");
    println!("The negative value of security through obscurity in the post-P world.\n");

    println!("CONTEXT: Late November 2026 saw an unprecedented cluster of");
    println!("AI-generated mathematical results that challenge the assumptions");
    println!("underpinning public-key cryptography:\n");

    println!("  REAL (published, Lean-verified where noted):");
    println!("  - VP != VNP over every field (OpenAI Nov 23, Anthropic/Claude Nov 23)");
    println!("  - Matrix multiplication exponent omega <= 9/4 (OpenAI Nov 24)");
    println!("  - Unconditional L_N[1/3, O(1)] factoring (Google Cogentic/Gemini 4 Nov 25)");
    println!();
    println!("  RUMORED (Aaronson: \"virtually certain\" from multiple sources):");
    println!("  - Polynomial-time integer factorization at OpenAI and/or Anthropic");
    println!("  - Possible results for elliptic curve discrete log and Ring-LWE");
    println!("  - Aaronson's caveat: \"A rumor is a rumor... falls well short of Factoring in P\"");
    println!();

    demonstrate_complexity_gap();
    demonstrate_obscurity_theater();
    demonstrate_rsa_vs_ecc();
    demonstrate_impairment_test();

    println!("\n--- SUMMARY ---");
    println!("1. Google's L_N[1/3, 105] is theoretical, not practical. Same alpha as GNFS.");
    println!("2. RSA (factoring) != Bitcoin/ECC (discrete log). Different problems.");
    println!("3. Obscurity layers cost money and fail silently. They are negative value.");
    println!("4. Real security: access control, rotation, audit trails — not naming conventions.");
    println!("5. If P=NP or AI finds poly-time shortcuts, hardness-based crypto faces");
    println!("   impairment. Process-based controls survive. Plan accordingly.");
    println!();
    println!("PRIMARY SOURCES: see module docs at top of file.");
}
