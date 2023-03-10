pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&i| i != 0)
        .filter(|&&i| i < limit)
        .map(|&i| (i..limit).step_by(i as usize).collect::<Vec<u32>>())
        .flatten()
        .collect::<std::collections::HashSet<u32>>()
        .iter()
        .sum()
}
