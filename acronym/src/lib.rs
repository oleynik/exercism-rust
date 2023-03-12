pub fn abbreviate(phrase: &str) -> String {
    if phrase.contains(":") {
        return phrase.split(":").nth(0).map(String::from).unwrap();
    }
    phrase
        .split([' ', '-', '_'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            if w == w.to_uppercase() {
                w.chars().nth(0).map(String::from)
            } else {
                Some(w.to_string())
            }
        })
        .filter(|w| w.is_some())
        .map(Option::unwrap)
        .map(|w| {
            w.chars()
                .enumerate()
                .filter(|(i, c)| *i == 0 || c.is_uppercase())
                .map(|x| x.1)
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(char::to_uppercase)
        .map(|c| c.to_string())
        .collect()
}
