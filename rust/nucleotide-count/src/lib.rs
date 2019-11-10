use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        Err(nucleotide)
    } else if let Some(c) = dna.chars().find(|&c| !"ACGT".contains(c)) {
        Err(c)
    } else {
        Ok(dna.chars().filter(|c| c == &nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(c) = dna.chars().find(|&c| !"ACGT".contains(c)) {
        Err(c)
    } else {
        Ok("ACGT"
            .chars()
            .map(|n| (n, dna.chars().filter(|c| c == &n).count()))
            .collect::<HashMap<char, usize>>())
    }
}
