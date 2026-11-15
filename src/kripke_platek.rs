/// Kripke-Platek Set Theory & Type-Theoretic Grounding
/// Demonstrates how KP axioms construct types from the empty set,
/// and why this is the minimal substrate for tuples, functions, and computation.

use std::collections::BTreeSet;
use std::fmt;

/// A hereditarily finite set (HFS) — the universe of KP without infinity.
/// Every element is itself a hereditarily finite set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HFS(BTreeSet<HFS>);

impl fmt::Display for HFS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, elem) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl HFS {
    /// The empty set ∅ — Layer 0: Unit type.
    pub fn empty() -> Self {
        HFS(BTreeSet::new())
    }

    /// Pairing axiom: {a, b}
    pub fn pair(a: &HFS, b: &HFS) -> Self {
        let mut s = BTreeSet::new();
        s.insert(a.clone());
        s.insert(b.clone());
        HFS(s)
    }

    /// Singleton: {a}
    pub fn singleton(a: &HFS) -> Self {
        let mut s = BTreeSet::new();
        s.insert(a.clone());
        HFS(s)
    }

    /// Union axiom: ⋃a = {x | ∃y ∈ a, x ∈ y}
    pub fn union(&self) -> Self {
        let mut result = BTreeSet::new();
        for elem in &self.0 {
            for sub in &elem.0 {
                result.insert(sub.clone());
            }
        }
        HFS(result)
    }

    /// Successor: succ(n) = n ∪ {n} (for von Neumann ordinals)
    pub fn successor(&self) -> Self {
        let mut result = self.0.clone();
        result.insert(self.clone());
        HFS(result)
    }

    /// n-th von Neumann ordinal: n = {0, 1, ..., n-1}
    pub fn ordinal(n: usize) -> Self {
        let mut current = HFS::empty();
        for _ in 0..n {
            current = current.successor();
        }
        current
    }

    /// Kuratowski ordered pair: (a, b) = {{a}, {a, b}}
    pub fn kuratowski_pair(a: &HFS, b: &HFS) -> Self {
        let singleton_a = HFS::singleton(a); // {a}
        let pair_ab = HFS::pair(a, b);       // {a, b}
        HFS::pair(&singleton_a, &pair_ab)    // {{a}, {a, b}}
    }

    /// n-tuple via nested Kuratowski pairs: (a₁, ..., aₙ) = (a₁, (a₂, ..., aₙ))
    pub fn tuple(elements: &[HFS]) -> Self {
        match elements.len() {
            0 => HFS::empty(),
            1 => elements[0].clone(),
            _ => {
                let rest = HFS::tuple(&elements[1..]);
                HFS::kuratowski_pair(&elements[0], &rest)
            }
        }
    }

    /// Cartesian product A × B using the KP construction:
    /// A × B := ⋃{A × {b} | b ∈ B}
    pub fn cartesian_product(a: &HFS, b: &HFS) -> Self {
        // For each b' in B, form A × {b'} = {(x, b') | x ∈ A}
        let mut slices = BTreeSet::new();
        for b_elem in &b.0 {
            let singleton_b = HFS::singleton(b_elem);
            let mut slice = BTreeSet::new();
            for a_elem in &a.0 {
                let pair = HFS::kuratowski_pair(a_elem, b_elem);
                slice.insert(pair);
            }
            slices.insert(HFS(slice));
        }
        // Take the union of all slices
        HFS(slices).union()
    }

    /// Check if this set is a valid Kuratowski ordered pair.
    /// Returns (first, second) if it matches {{a}, {a, b}} or {{a}} (when a=b).
    pub fn as_ordered_pair(&self) -> Option<(HFS, HFS)> {
        // Case 1: a = b, so (a, a) = {{a}} — a set containing one singleton
        if self.0.len() == 1 {
            let elem = self.0.iter().next().unwrap();
            if elem.0.len() == 1 {
                let a = elem.0.iter().next().unwrap().clone();
                return Some((a.clone(), a));
            }
            return None;
        }

        // Case 2: a ≠ b, so (a, b) = {{a}, {a, b}} — two elements
        if self.0.len() != 2 {
            return None;
        }
        let elems: Vec<&HFS> = self.0.iter().collect();
        for (i, j) in [(0, 1), (1, 0)] {
            if elems[i].0.len() == 1 {
                let a = elems[i].0.iter().next().unwrap().clone();
                let other = &elems[j].0;
                if other.len() == 2 && other.contains(&a) {
                    let b = other.iter().find(|x| *x != &a).unwrap().clone();
                    return Some((a, b));
                }
            }
        }
        None
    }

    /// Cardinality (number of direct elements)
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub fn show_kripke_platek_station() {
    println!("=== KRIPKE-PLATEK SET THEORY STATION ===");
    println!();
    print_overview();
    println!();
    demonstrate_construction_layers();
    println!();
    demonstrate_type_assignments();
    println!();
    demonstrate_peg_in_kp();
}

fn print_overview() {
    println!("Kripke-Platek Set Theory (1960s)");
    println!("Developed by Saul Kripke and Richard Platek");
    println!();
    println!("KP is the weakest reasonable set theory. It's roughly the predicative");
    println!("part of ZFC — no power sets, no full replacement, no axiom of choice.");
    println!("What it gives you is exactly enough to build computation:");
    println!();
    println!("  • Empty set          → Unit type ()");
    println!("  • Pairing            → Sum type A + B");
    println!("  • Union              → Dependent sum / list monad join");
    println!("  • ∆₀-Separation      → Filter / refinement type");
    println!("  • ∆₀-Collection      → Image / bounded existential");
    println!("  • Foundation         → Well-founded induction");
    println!("  • Infinity (KPω)     → Natural numbers ℕ with induction");
    println!();
    println!("Every set in KP has a finite derivation history from ∅.");
    println!("No impredicative circularity. No non-constructive existence.");
    println!("This is type theory wearing set-theoretic clothing.");
}

fn demonstrate_construction_layers() {
    println!("--- DEMONSTRATION: Construction Layers (What Builds First) ---");
    println!();

    // Layer 0: Unit
    println!("LAYER 0: Unit Type (Empty Set)");
    let empty = HFS::empty();
    println!("  ∅ = {}", empty);
    println!("  Type: Unit — the 0-tuple, the trivial parse, no information.");
    println!();

    // Layer 1: Natural Numbers via von Neumann ordinals
    println!("LAYER 1: Natural Numbers (von Neumann Ordinals)");
    println!("  Built by repeated pairing/successor from ∅:");
    for n in 0..=5 {
        let ordinal = HFS::ordinal(n);
        println!("  {} = {}", n, ordinal);
    }
    println!("  Each n = {{0, 1, ..., n-1}}. Foundation guarantees finite derivation.");
    println!();

    // Layer 2: Product Types (Tuples) via Kuratowski pairs
    println!("LAYER 2: Product Types (Tuples)");
    let a = HFS::ordinal(1); // {∅}
    let b = HFS::ordinal(2); // {∅, {∅}}
    let c = HFS::ordinal(3); // {∅, {∅}, {∅, {∅}}}

    let pair_ab = HFS::kuratowski_pair(&a, &b);
    println!("  Kuratowski ordered pair (a, b) = {{{{a}}, {{a, b}}}}");
    println!("  (1, 2) = {}", pair_ab);
    println!();

    // Verify order matters
    let pair_ba = HFS::kuratowski_pair(&b, &a);
    println!("  Order matters: (1, 2) ≠ (2, 1)");
    println!("  (1, 2) = {}", pair_ab);
    println!("  (2, 1) = {}", pair_ba);
    assert_ne!(pair_ab, pair_ba);
    println!();

    // 3-tuple via nesting
    let triple = HFS::tuple(&[a.clone(), b.clone(), c.clone()]);
    println!("  3-tuple (1, 2, 3) = (1, (2, 3)) = {}", triple);
    println!();

    // Verify we can decode it back
    if let Some((first, rest)) = triple.as_ordered_pair() {
        println!("  Decoded: first = {}, rest = (2, 3)", first);
        if let Some((second, third)) = rest.as_ordered_pair() {
            println!("  Fully decoded: ({}, {}, {})", first, second, third);
        }
    }
    println!();

    // Layer 3: Cartesian Products
    println!("LAYER 3: Cartesian Products (Functions as Sets of Pairs)");
    let two = HFS::ordinal(2); // {∅, {∅}} — represents {0, 1}
    let three = HFS::ordinal(3); // {∅, {∅}, {∅, {∅}}} — represents {0, 1, 2}

    let product = HFS::cartesian_product(&two, &three);
    println!("  2 × 3 has {} elements (ordered pairs):", product.len());
    for elem in product.0.iter().take(6) {
        if let Some((x, y)) = elem.as_ordered_pair() {
            println!("    ({}, {})", x, y);
        }
    }
    println!();
    println!("  A function f: A → B is a subset of A × B satisfying functionality.");
    println!("  Type theory reading: function type A → B.");
    println!();

    // Layer 4: Sum Types
    println!("LAYER 4: Sum Types (Tagged Unions)");
    let zero = HFS::ordinal(0);
    let one = HFS::ordinal(1);
    let tag_a = HFS::kuratowski_pair(&zero, &a); // inl(a) = (0, a)
    let tag_b = HFS::kuratowski_pair(&one, &b);  // inr(b) = (1, b)
    println!("  A + B = ({{0}} × A) ∪ ({{1}} × B)");
    println!("  inl(1) = (0, 1) = {}", tag_a);
    println!("  inr(2) = (1, 2) = {}", tag_b);
    println!("  Tags make it a proper coproduct — you can always tell which side.");
}

fn demonstrate_type_assignments() {
    println!("--- DEMONSTRATION: KP Axioms as Type Rules ---");
    println!();
    println!("Each KP axiom corresponds to an introduction/elimination rule:");
    println!();
    println!("  AXIOM              TYPE-THEORETIC READING        CONSTRUCT");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("  Extensionality     Identity by structure         Equality principle");
    println!("  Empty Set          ∃a: Set, ∀x, x∉a              Unit type ()");
    println!("  Pairing            ∀a b, ∃c, c≅{{a,b}}           Sum type A + B");
    println!("  Union              ∀a, ∃b, b=⋃a                  Dependent sum Σ");
    println!("  ∆₀-Separation      Filter by local predicate     Refinement type");
    println!("  ∆₀-Collection      Image under ∆₀ relation       Bounded ∃ / image");
    println!("  Foundation         Every non-empty set has min   Well-founded induction");
    println!("  Infinity (KPω)     ∃ inductive set               ℕ with induction");
    println!();
    println!("The ∆₀ restriction is the key: predicates can only quantify over");
    println!("elements of existing sets, not the whole universe. This is what");
    println!("makes KP predicative — no circular definitions, no impredicativity.");
    println!();

    // Demonstrate ∆₀-separation conceptually
    println!("Example: ∆₀-Separation as Filter");
    let five = HFS::ordinal(5); // {0, 1, 2, 3, 4}
    println!("  Let a = 5 = {}", five);
    println!("  Filter: keep elements x where x is even (x ∈ {{0, 2, 4}})");

    // Manual filter for demonstration
    let evens = HFS::ordinal(3); // {0, 1, 2} represents indices 0,1,2 → values 0,2,4
    let mut filtered = BTreeSet::new();
    for elem in &five.0 {
        // Check if ordinal value is even (simplified: check cardinality)
        if elem.len() % 2 == 0 {
            filtered.insert(elem.clone());
        }
    }
    let result = HFS(filtered);
    println!("  Result: {{x ∈ 5 | x is even}} = {}", result);
    println!();
    println!("The predicate 'x is even' is ∆₀ — it only inspects x's structure,");
    println!("it doesn't quantify over all sets. This is local, checkable, honest.");
}

fn demonstrate_peg_in_kp() {
    println!("--- DEMONSTRATION: PEG Grammars Live in KP ---");
    println!();
    println!("A PEG grammar is a 4-tuple (N, Σ, P, e_S). Each component is");
    println!("constructible in KP from ∅ via pairing and union:");
    println!();

    // Build a tiny example: N = {Expr}, Σ = {'0', '+', '('}
    let n_expr = HFS::ordinal(1); // represents nonterminal "Expr"
    let sigma_0 = HFS::ordinal(2); // represents terminal '0'
    let sigma_plus = HFS::ordinal(3); // represents terminal '+'
    let sigma_lparen = HFS::ordinal(4); // represents terminal '('

    println!("  N = {{Expr}}        (set of nonterminals)");
    println!("  Σ = {{'0', '+', '('}} (terminal alphabet)");
    println!();

    // P: N → Expr is a function, represented as a set of pairs
    let expr_rule = HFS::ordinal(5); // represents some parsing expression
    let p_pair = HFS::kuratowski_pair(&n_expr, &expr_rule);
    let p_function = HFS::singleton(&p_pair); // {(Expr, rule)}
    println!("  P: N → Expr       (production function as set of pairs)");
    println!("  P = {{(Expr, rule)}} = {}", p_function);
    println!();

    // e_S: start expression
    let e_s = expr_rule.clone();
    println!("  e_S = rule        (start expression)");
    println!();

    // The whole grammar as a 4-tuple
    let n_set = HFS::singleton(&n_expr);
    let sigma_set = HFS::pair(&HFS::pair(&sigma_0, &sigma_plus), &sigma_lparen);
    let grammar = HFS::tuple(&[n_set, sigma_set, p_function, e_s]);

    println!("  Grammar = (N, Σ, P, e_S) as a 4-tuple:");
    println!("  {}", grammar);
    println!();
    println!("Every parse decision traces back to a specific rule at a specific");
    println!("position with a specific alternative order. The tuple structure");
    println!("makes the derivation path explicit and inspectable.");
    println!();
    println!("KP gives you this without power sets, without choice, without");
    println!("any of ZFC's infinite baggage. Just ∅, pairing, and induction.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_set_is_unique() {
        // Extensionality: two empty sets are equal
        assert_eq!(HFS::empty(), HFS::empty());
    }

    #[test]
    fn test_von_neumann_ordinals() {
        let zero = HFS::ordinal(0);
        let one = HFS::ordinal(1);
        let two = HFS::ordinal(2);
        let three = HFS::ordinal(3);

        assert!(zero.is_empty());
        assert_eq!(one.len(), 1);
        assert_eq!(two.len(), 2);
        assert_eq!(three.len(), 3);

        // Each ordinal contains all smaller ordinals
        assert!(one.0.contains(&zero));
        assert!(two.0.contains(&zero));
        assert!(two.0.contains(&one));
        assert!(three.0.contains(&two));
    }

    #[test]
    fn test_kuratowski_pair_preserves_order() {
        let a = HFS::ordinal(1);
        let b = HFS::ordinal(2);

        let ab = HFS::kuratowski_pair(&a, &b);
        let ba = HFS::kuratowski_pair(&b, &a);

        // Order matters: (a,b) ≠ (b,a) unless a = b
        assert_ne!(ab, ba);

        // But (a,b) = (c,d) iff a=c and b=d
        let ab2 = HFS::kuratowski_pair(&a, &b);
        assert_eq!(ab, ab2);
    }

    #[test]
    fn test_kuratowski_pair_decoding() {
        let a = HFS::ordinal(1);
        let b = HFS::ordinal(2);
        let pair = HFS::kuratowski_pair(&a, &b);

        let decoded = pair.as_ordered_pair();
        assert!(decoded.is_some());
        let (first, second) = decoded.unwrap();
        assert_eq!(first, a);
        assert_eq!(second, b);
    }

    #[test]
    fn test_tuple_roundtrip() {
        let a = HFS::ordinal(1);
        let b = HFS::ordinal(2);
        let c = HFS::ordinal(3);

        let triple = HFS::tuple(&[a.clone(), b.clone(), c.clone()]);

        // Decode: (a, (b, c))
        if let Some((first, rest)) = triple.as_ordered_pair() {
            assert_eq!(first, a);
            if let Some((second, third)) = rest.as_ordered_pair() {
                assert_eq!(second, b);
                assert_eq!(third, c);
            } else {
                panic!("Failed to decode nested pair");
            }
        } else {
            panic!("Failed to decode outer pair");
        }
    }

    #[test]
    fn test_cartesian_product_size() {
        let two = HFS::ordinal(2);   // 2 elements
        let three = HFS::ordinal(3); // 3 elements

        let product = HFS::cartesian_product(&two, &three);
        assert_eq!(product.len(), 6); // 2 × 3 = 6 pairs
    }

    #[test]
    fn test_cartesian_product_elements_are_pairs() {
        let two = HFS::ordinal(2);
        let three = HFS::ordinal(3);

        let product = HFS::cartesian_product(&two, &three);
        for elem in product.0.iter() {
            assert!(elem.as_ordered_pair().is_some());
        }
    }

    #[test]
    fn test_union_flattens_one_level() {
        // ⋃{{a}, {b}} = {a, b}
        let a = HFS::ordinal(1);
        let b = HFS::ordinal(2);
        let nested = HFS::pair(&HFS::singleton(&a), &HFS::singleton(&b));

        let flattened = nested.union();
        assert_eq!(flattened, HFS::pair(&a, &b));
    }

    #[test]
    fn test_successor_operation() {
        let zero = HFS::empty();
        let one = zero.successor();
        let two = one.successor();

        assert_eq!(one, HFS::ordinal(1));
        assert_eq!(two, HFS::ordinal(2));
    }

    #[test]
    fn test_peg_grammar_is_constructible() {
        // A PEG grammar (N, Σ, P, e_S) is a 4-tuple of KP-constructible objects
        let n = HFS::singleton(&HFS::ordinal(1));
        let sigma = HFS::pair(&HFS::ordinal(2), &HFS::ordinal(3));
        let p = HFS::singleton(&HFS::kuratowski_pair(
            &HFS::ordinal(1),
            &HFS::ordinal(4),
        ));
        let e_s = HFS::ordinal(4);

        let grammar = HFS::tuple(&[n, sigma, p, e_s]);
        assert!(!grammar.is_empty());

        // The grammar is a valid KP object: hereditarily finite, well-founded
        // (guaranteed by construction from ∅ via pairing)
    }
}
