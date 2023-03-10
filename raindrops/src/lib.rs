pub fn raindrops(n: u32) -> String {
    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|e| n % e.0 == 0)
        .map(|(_, v)| *v)
        .collect();
    match result.len() {
        0 => n.to_string(),
        _ => result,
    }
}
