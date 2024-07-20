use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut result = String::from(input);
    reverse_grapheme_clusters_in_place(&mut result);
    result
}
