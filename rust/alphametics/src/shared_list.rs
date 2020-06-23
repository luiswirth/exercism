/// Singly-linked list, linked with shared pointers.
/// Use case: chain stack pointers in nested function calls.
///
/// In this exercise, it is used to recursively apply
/// partial solutions and see if they work.
pub enum SharedList<'a, T> {
    Nil,
    Node(T, &'a SharedList<'a, T>),
}

impl<'a, T> SharedList<'a, T> {
    pub fn new() -> Self {
        SharedList::Nil
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self)
    }

    pub fn prepend(&'a self, item: T) -> Self {
        SharedList::Node(item, self)
    }
}

pub struct Iter<'a, T>(&'a SharedList<'a, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            SharedList::Node(item, next) => {
                self.0 = next;
                Some(item)
            }
            SharedList::Nil => None,
        }
    }
}
