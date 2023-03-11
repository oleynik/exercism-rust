pub fn brackets_are_balanced(string: &str) -> bool {
    let map = std::collections::HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
    let mut stack = vec![];
    for ch in string.chars() {
        if map.contains_key(&ch) {
            stack.push(*map.get(&ch).unwrap());
            continue;
        }
        if !stack.is_empty() && *stack.last().unwrap() == ch {
            stack.pop();
            continue;
        }
        if map.values().any(|&c| c == ch) {
            return false;
        }
    }
    stack.is_empty()
}
