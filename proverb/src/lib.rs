pub fn build_proverb(list: &[&str]) -> String {
    match list.is_empty() {
        true => String::new(),
        false => list
            .windows(2)
            .map(|item|format!("For want of a {} the {} was lost.\n", item[0], item[1]))
            .collect::<String>() + format!("And all for the want of a {}.", list.first().unwrap()).as_str()
    }
}
