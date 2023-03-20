use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // Validation DNA
    for c in dna.chars() {
        if !NUCLEOTIDES.contains(c) {
            return Err(c);
        }
    }
    match NUCLEOTIDES.contains(nucleotide) {
        false => Err(nucleotide),
        true => Ok(dna.chars().filter(|c| c == &nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    for c in NUCLEOTIDES.chars() {
        let count = count(c, dna)?;
        result.insert(c, count);
    }
    Ok(result)
}
