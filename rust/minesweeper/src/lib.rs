pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len();
    (0..height)
        .map(|y| {
            let width = field[y as usize].len();
            (0..width)
                .map(|x| {
                    if field[y].as_bytes()[x] == b'*' {
                        '*'
                    } else {
                        match field
                            .iter()
                            .take(y + 2)
                            .skip((y as isize - 1).max(0) as usize)
                            .flat_map(|&line| {
                                line.chars()
                                    .take(x + 2)
                                    .skip((x as isize - 1).max(0) as usize)
                            })
                            .filter(|&c| c == '*')
                            .count()
                        {
                            0 => ' ',
                            n => std::char::from_digit(n as u32, 10).unwrap(),
                        }
                    }
                })
                .collect()
        })
        .collect()
}
