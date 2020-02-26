use std::collections::HashMap;
use std::cell::RefCell;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    type Line = Vec<char>;

    let (lhs, rhs) = {
        let s = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let mut iter = s.splitn(2, "==");
        (
            iter.next().unwrap().to_string(),
            iter.next().unwrap().to_string(),
        )
    };

    let lhs = {
        let mut lines = Vec::new();
        for line in lhs.split('+') {
            lines.push(line.chars().rev().collect::<Line>());
        }
        lines
    };

    let rhs = rhs.chars().rev().collect::<Line>();

    let mut map = RefCell::new(input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c, 0))
        .collect::<HashMap<char, u8>>());

    for char_val in map.borrow_mut().values_mut() {
        for new_val in 0..9 {
            *char_val = new_val;

            for i in 0..rhs.len() {
                if lhs
                    .iter()
                    .map(|line| line.get(i).expect("if none stop"))
                    .map(|c| map.borrow().get(c).unwrap())
                    .sum::<u8>()
                    == *map.borrow().get(rhs.get(i).unwrap()).unwrap()
                {}
            }
        }
    }

    Some(map)
}
