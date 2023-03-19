pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::new(); digits.len() + 1],
        _ => digits
            .chars()
            .map(String::from)
            .collect::<Vec<String>>()
            .windows(len)
            .map(|a| a.concat())
            .collect::<Vec<String>>(),
    }
}
