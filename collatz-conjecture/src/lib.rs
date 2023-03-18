pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut number = n;
    let mut result = 0;
    while number > 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            let mut overflow = false;
            (number, overflow) = number.overflowing_mul(3);
            if overflow {
                return None;
            }
            (number, overflow) = number.overflowing_add(1);
            if overflow {
                return None;
            }
        }
        result += 1;
    }
    Some(result)
}
