pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remainding = n;

    loop {
        if remainding == 1 {
            break;
        }
        for factor in 2.. {
            if remainding % factor == 0 {
                factors.push(factor);
                remainding /= factor;
                break;
            }
        }
    }
    factors
}
