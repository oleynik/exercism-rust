pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result = vec![];
    (2..=upper_bound).for_each(|i| {
        if !result.iter().any(|e| i % e == 0) {
            result.push(i);
        }
    });
    result
}
