/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut result = 0;
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    for i in 0..s1.len() {
        if b1[i] != b2[i] {
            result += 1;
        }
    }
    Some(result)
}
