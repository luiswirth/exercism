/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code = code.to_string();
    code.retain(|c| !c.is_whitespace());
    if code.len() <= 1 || code.chars().any(|c| c.to_digit(10).is_none()) {
        return false;
    }
    code.chars()
        .rev()
        .enumerate()
        .map(|(n, c)| {
            let p = c.to_digit(10).unwrap();
            if n % 2 == 1 {
                let d = p * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                p
            }
        })
        .sum::<u32>()
        % 10
        == 0
}
