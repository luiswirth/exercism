pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();
    for factor in factors {
        if factor == &0 {
            continue;
        }
        for i in 1.. {
            if factor * i < limit {
                multiples.push(factor * i);
            } else {
                break;
            }
        }
    }
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
}
