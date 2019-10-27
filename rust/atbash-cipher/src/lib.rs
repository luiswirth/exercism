/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new(); 
    let plain = plain.to_lowercase().chars().filter(|c| c.is_ascii() && c.is_alphanumeric()).collect::<String>();
    for (n, mut c) in plain.chars().enumerate() {
	if c.is_alphabetic() {
	    let i = c as u32 - 'a' as u32;
	    let j = 'z' as u32 - i;
	    c = std::char::from_u32(j).unwrap();
	} 
	if n % 5 == 0 && n != 0 {
	    result.push(' ');
	}
	result.push(c);
    };
    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::new(); 
    let cipher = cipher.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    for mut c in cipher.chars() {
	if c.is_alphabetic() {
	    let i = c as u32 - 'a' as u32;
	    let j = 'z' as u32 - i;
	    c = std::char::from_u32(j).unwrap();
	}
	result.push(c);
    };
    result
}



