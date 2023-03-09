pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let mut sum = 0u64;
    for char in num_string.chars() {
        let digit = char.to_digit(10).unwrap();
        let power = digit.pow(num_string.len() as u32) as u64;
        match sum.checked_add(power) {
            Some(res) if res > num as u64 => return false,
            Some(res) => sum = res,
            None => return false
        }
    }
    num == sum as u32
}
