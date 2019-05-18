use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if NUCLEOTIDES.contains(&nucleotide) {
        nucleotide_counts(dna).map(|counts| counts[&nucleotide])
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counter: HashMap<char, usize> = NUCLEOTIDES.iter().map(|&n| (n, 0usize)).collect();

    for c in dna.chars() {
        if NUCLEOTIDES.contains(&c) {
            *counter.entry(c).or_insert(0) += 1;
        } else {
            return Err(c);
        }
    }

    Ok(counter)
}
