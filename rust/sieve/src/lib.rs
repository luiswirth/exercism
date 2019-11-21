pub fn primes_up_to(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut candidates: Vec<_> = (2..=limit).rev().collect();
    while let Some(prime) = candidates.pop() {
        primes.push(prime);
        candidates.retain(|n| n % prime != 0);
    }
    primes
}
