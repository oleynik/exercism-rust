use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    proteins: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.proteins.get(codon) {
            Some(&p) => Some(p),
            None => None,
        }
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
    let mut proteins = HashMap::with_capacity(pairs.len());
    pairs.iter().for_each(|(c, p)| {
        proteins.insert(*c, *p);
    });
    CodonsInfo { proteins }
}
