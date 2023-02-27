#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first = _first_list.len();
    let second = _second_list.len();

    // 0. 1.len == 2.len:   Potentially Equal
    if first == second
        && equal(_first_list, _second_list) {
        return Comparison::Equal;
    }

    // 1. 1.len < 2.len:    Potentially SubList
    if first < second
        && (first == 0 || partially_equal(_first_list, _second_list)) {
        return Comparison::Sublist;
    }

    // 2. 1.len > 2.len:    Potentially SuperList
    if first > second
        && (second == 0 || partially_equal(_second_list, _first_list)) {
        return Comparison::Superlist;
    }

    // 3. Otherwise:        UnEqual
    Comparison::Unequal
}

fn equal<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    for i in 0..first.len() {
        if first[i] != second[i] {
            return false
        }
    }
    true
}

fn partially_equal<T: PartialEq>(smaller: &[T], bigger: &[T]) -> bool {
    for i in 0..bigger.len()-smaller.len()+1 {
        if smaller[0] == bigger[i] {
            if equal(smaller, &bigger[i..i+smaller.len()]) {
                return true
            }
        }
    }
    false
}