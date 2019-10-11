pub fn is_prime(c: u32) -> bool {
    !(2..c-1).any(|i| c % i == 0) // any returns ture if any element satifies the predicate (closure)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}
