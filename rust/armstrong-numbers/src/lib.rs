pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    num == digits.iter().map(|d| d.pow(digits.len() as u32)).sum()
}
