/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match code.trim().len() {
        0 | 1 => false,
        _ => {
            match code
                .chars()
                .filter(|&c| c != ' ')
                .all(|c| c.is_ascii_digit())
            {
                false => false,
                true => {
                    let sum: u32 = code
                        .chars()
                        .filter(|&c| c != ' ')
                        .rev()
                        .enumerate()
                        .map(|(i, ch)| {
                            if i % 2 == 0 {
                                ch.to_digit(10).unwrap()
                            } else {
                                let digit = ch.to_digit(10).unwrap() * 2;
                                if digit > 9 {
                                    digit - 9
                                } else {
                                    digit
                                }
                            }
                        })
                        .sum();
                    sum % 10 == 0
                }
            }
        }
    }
}
