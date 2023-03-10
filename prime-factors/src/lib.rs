pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut candidate = n;
    while candidate != 1 {
        for i in *result.last().unwrap_or_else(||&2)..=candidate {
            if candidate % i == 0 {
                result.push(i);
                candidate /= i;
                break;
            }
        }
    }
    result
}
