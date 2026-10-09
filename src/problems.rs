//! NP-hard problem instances for testing Grover vs classical approaches.

/// Subset sum problem: find subset of numbers that sums to target.
/// Classical approach: exponential backtracking.
/// Grover approach: encode all subsets as search space.
pub fn subset_sum_classical(numbers: &[i32], target: i32) -> Option<Vec<usize>> {
    let n = numbers.len();
    // Try all subsets via bitmask (exponential in n)
    for mask in 0..(1 << n) {
        let mut sum = 0;
        let mut indices = Vec::new();
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                sum += numbers[i];
                indices.push(i);
            }
        }
        if sum == target {
            return Some(indices);
        }
    }
    None
}

/// Boolean satisfiability: find assignment that satisfies formula.
/// Using simple DPLL-style backtracking for classical approach.
pub fn sat_classical(clauses: &[Vec<i32>], n_vars: usize) -> Option<Vec<bool>> {
    let mut assignment = vec![false; n_vars];

    fn is_satisfied(clauses: &[Vec<i32>], assignment: &[bool]) -> bool {
        clauses.iter().all(|clause| {
            clause.iter().any(|lit| {
                let var = lit.abs() as usize - 1;
                if *lit > 0 {
                    assignment[var]
                } else {
                    !assignment[var]
                }
            })
        })
    }

    fn dpll(clauses: &[Vec<i32>], assignment: &mut [bool], var: usize) -> bool {
        if is_satisfied(clauses, assignment) {
            return true;
        }
        if var >= assignment.len() {
            return false;
        }

        // Try true
        assignment[var] = true;
        if dpll(clauses, assignment, var + 1) {
            return true;
        }

        // Try false
        assignment[var] = false;
        if dpll(clauses, assignment, var + 1) {
            return true;
        }

        false
    }

    if dpll(clauses, &mut assignment, 0) {
        Some(assignment)
    } else {
        None
    }
}

/// Traveling Salesman Problem: find shortest tour visiting all cities.
/// Using nearest-neighbor heuristic for classical approach.
pub fn tsp_nearest_neighbor(distances: &[Vec<u64>]) -> (Vec<usize>, u64) {
    let n = distances.len();
    let mut visited = vec![false; n];
    let mut tour = Vec::with_capacity(n);
    let mut current = 0;
    visited[0] = true;
    tour.push(0);

    for _ in 1..n {
        let mut best_next = None;
        let mut best_dist = u64::MAX;
        for next in 0..n {
            if !visited[next] && distances[current][next] < best_dist {
                best_dist = distances[current][next];
                best_next = Some(next);
            }
        }
        if let Some(next) = best_next {
            visited[next] = true;
            tour.push(next);
            current = next;
        }
    }

    // Calculate total distance
    let mut total = 0;
    for i in 0..n - 1 {
        total += distances[tour[i]][tour[i + 1]];
    }
    total += distances[tour[n - 1]][tour[0]];

    (tour, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_sum() {
        let result = subset_sum_classical(&[3, 34, 4, 5, 6], 9);
        assert!(result.is_some());
    }

    #[test]
    fn test_sat() {
        // (x1 OR x2) AND (NOT x1 OR x2) AND (x1 OR NOT x2)
        let clauses = vec![vec![1, 2], vec![-1, 2], vec![1, -2]];
        let result = sat_classical(&clauses, 2);
        assert!(result.is_some());
    }

    #[test]
    fn test_tsp() {
        // Simple 3-city TSP
        let distances = vec![
            vec![0, 10, 15],
            vec![10, 0, 20],
            vec![15, 20, 0],
        ];
        let (tour, _dist) = tsp_nearest_neighbor(&distances);
        assert_eq!(tour.len(), 3);
    }
}