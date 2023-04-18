use std::cmp::Ordering;

pub fn find<T: Ord, C: AsRef<[T]>>(array: C, key: T) -> Option<usize> {
    let len = array.as_ref().len();
    if len == 0 {
        None
    } else {
        let mid = len / 2;
        match key.cmp(&array.as_ref()[mid]) {
            Ordering::Less => find(&array.as_ref()[0..mid], key),
            Ordering::Greater => match find(&array.as_ref()[mid + 1..], key) {
                Some(idx) => Some(idx + mid + 1),
                None => None,
            },
            Ordering::Equal => Some(mid),
        }
    }
}
