#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let sub_list = first.is_empty() || second.windows(first.len()).any(|v| v == first);
    let super_list = second.is_empty() || first.windows(second.len()).any(|v| v == second);
    use Comparison::*;
    match (sub_list, super_list) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        (false, false) => Unequal,
    }
}
