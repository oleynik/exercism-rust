#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    const NUCLEOTIDES: &'static str = "ACGT";
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let result = dna
            .to_uppercase()
            .char_indices()
            .map(|(i, c)| {
                if Self::NUCLEOTIDES.contains(c) {
                    Ok(c)
                } else {
                    Err(i)
                }
            })
            .collect::<Result<String, _>>();
        match result {
            Ok(s) => Ok(Dna(s)),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .0
            .chars()
            .map(|c| match c {
                'A' => Ok('U'),
                'C' => Ok('G'),
                'G' => Ok('C'),
                'T' => Ok('A'),
                c => Err(format!("Unknown nucleotide {}", c)),
            })
            .filter_map(|r| r.ok())
            .collect();
        Rna::new(rna.as_str()).unwrap()
    }
}

impl Rna {
    const NUCLEOTIDES: &'static str = "ACGU";
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .to_uppercase()
            .char_indices()
            .map(|(i, c)| {
                if Self::NUCLEOTIDES.contains(c) {
                    Ok(c)
                } else {
                    Err(i)
                }
            })
            .collect::<Result<String, _>>()
        {
            Ok(s) => Ok(Self(s)),
            Err(i) => Err(i),
        }
    }
}
