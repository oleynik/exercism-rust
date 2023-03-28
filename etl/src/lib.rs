use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    h.iter()
        .map(|(k, v)| (v.iter().collect::<String>().to_lowercase(), k))
        .for_each(|(s, &i)| {
            s.chars().for_each(|c| {
                result.insert(c, i);
            })
        });
    result
}
