/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut valid = true;
    let digits = isbn
        .chars()
        .filter(|c| c != &'-')
        .map(|c| match c {
            c if c.is_digit(10) => c.to_digit(10).unwrap(),
            c if c == 'X' => 10,
            _ => {
                valid = false;
                0
            }
        })
        .collect::<Vec<u32>>();
    valid
        && digits.len() == 10
        && digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, n)| if n == &10 {10} else{(i + 1) * *n as usize})
            .sum::<usize>() % 11 == 0
}
