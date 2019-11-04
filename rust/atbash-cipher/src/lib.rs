/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_alphabetic() {
                let i = c as u8 - b'a';
                (b'z' - i) as char
            } else {
                c
            }
        })
        .enumerate()
        .flat_map(|(n, c)| {
            match n % 5 {
                0 if n > 0 => Some(' '),
                _ => None,
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_alphabetic() {
                let i = c as u8 - b'a';
                (b'z' - i) as char
            } else {
                c
            }
        })
        .collect::<String>()
}
