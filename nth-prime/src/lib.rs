pub fn nth(n: u32) -> u32 {
    let mut candidate = 2;
    let mut index = 0;
    'w: while index < n {
        candidate += 1;
        let limit = candidate >> 1;
        for i in 2..=limit {
            if candidate % i == 0 {
                continue 'w;
            }
        }
        index += 1;
    }
    candidate
}
