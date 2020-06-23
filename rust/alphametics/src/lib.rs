mod shared_list;

use shared_list::SharedList;
use std::collections::HashMap;

pub fn solve(equation: &str) -> Option<HashMap<char, u8>> {
    let operands: Vec<&str> = equation
        .split(|c: char| !c.is_alphabetic())
        .filter(|op| !op.is_empty())
        .collect();
    solve_inner(&operands, SharedList::new())
}

fn solve_inner(operands: &[&str], list: SharedList<(char, u8)>) -> Option<HashMap<char, u8>> {
    match solve_partial(operands, &list) {
        Err(()) => None,
        Ok(None) => Some(list.iter().cloned().collect()),
        Ok(Some(c)) => (0..10)
            .skip_while(|&x| x == 0 && operands.iter().any(|op| op.starts_with(c)))
            .filter(|&x| list.iter().all(|&(_, v)| v != x))
            .find_map(|x| solve_inner(operands, list.prepend((c, x)))),
    }
}

fn solve_partial(operands: &[&str], list: &SharedList<(char, u8)>) -> Result<Option<char>, ()> {
    let sum = operands.last().ok_or(())?;
    let summands = || operands.iter().rev().skip(1);

    let mut col_sum: u32 = 0;
    let mut col = 0;

    loop {
        let get_col = |w: &str| w.chars().rev().nth(col);

        for c in summands().filter_map(|op| get_col(op)) {
            match list.iter().find(|&&(k, _)| k == c) {
                None => return Ok(Some(c)),
                Some(&(_, digit)) => col_sum += digit as u32,
            }
        }

        match get_col(sum) {
            None => return Ok(None),
            Some(c) => match list.iter().find(|&&(k, _)| k == c) {
                None => return Ok(Some(c)),
                Some(&(_, digit)) => {
                    if col_sum % 10 != digit as u32 {
                        return Err(());
                    }
                }
            },
        };

        col += 1;
        col_sum /= 10; // carry to the next column
    }
}
