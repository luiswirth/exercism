#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(n).any(|v| v == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if b.windows(m).any(|v| v == a) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}
