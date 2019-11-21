#[derive(Debug, PartialEq)]
pub struct DNA {
    bases: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    bases: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if let Some(pos) = dna.chars().position(|c| !"ACGT".contains(c)) {
            Err(pos)
        } else {
            Ok(DNA {
                bases: dna.to_owned(),
            })
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            bases: self
                .bases
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => 'X',
                })
                .collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if let Some(pos) = rna.chars().position(|c| !"ACGU".contains(c)) {
            Err(pos)
        } else {
            Ok(RNA {
                bases: rna.to_owned(),
            })
        }
    }
}
