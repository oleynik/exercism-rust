pub struct CodonsInfo<'a> {
    proteins: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        for (c, p) in self.proteins.clone() {
            if c == codon {
                return Some(p);
            }
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result = vec![];
        let mut codon = String::new();
        for ch in rna.chars() {
            codon.push(ch);
            if codon.len() == 3 {
                let protein = self.name_for(codon.as_str())?;
                if protein.contains("stop") {
                    codon.clear();
                    break;
                }
                result.push(protein);
                codon.clear();
            }
        }
        if codon.len() == 0 {
            Some(result)
        } else {
            None
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { proteins: pairs }
}
