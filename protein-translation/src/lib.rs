use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_proteins: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        let p = self.codon_proteins.get(codon);
        match p {
            None => None,
            Some(v) => Some(*v),
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let codons = rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|a| a.into_iter().collect::<String>())
            .collect::<Vec<String>>();
        let mut proteins = vec![];
        for codon in codons {
            let s :&str = &codon;
            let protein = self.codon_proteins.get(s);
            match protein {
                None => return None,
                Some(&"stop codon") => break,
                Some(v) => proteins.push(*v)
            }
        }
        Some(proteins)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut ci = CodonsInfo {
        codon_proteins: HashMap::new(),
    };
    for (k, v) in pairs {
        ci.codon_proteins.insert(k, v);
    }
    ci
}
