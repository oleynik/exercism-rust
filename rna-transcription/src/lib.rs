#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

fn validate_nucleotides(nucleotides: &str, chain: &str) -> Result<String, usize> {
    chain
        .to_uppercase()
        .char_indices()
        .map(|(i, c)| {
            if nucleotides.contains(c) {
                Ok(c)
            } else {
                return Err(i);
            }
        })
        .collect()
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate_nucleotides("ACGT", dna).map(|r| Dna(r))
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .0
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                c => panic!("Unknown nucleotide {}", c),
            })
            .collect();
        Rna::new(rna.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate_nucleotides("ACGU", rna).map(|r| Rna(r))
    }
}
