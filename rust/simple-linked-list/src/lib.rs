use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        let mut current_node = &self.head;
        while let Some(inner_current_node) = current_node {
            len += 1;
            current_node = &inner_current_node.next;
        }
        len
    }

    pub fn push(&mut self, element: T) {
        let neck = self.head.take();
        self.head = Some(Box::new(Node {
            data: element,
            next: neck,
        }));
    }

    // removes and returns head element
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut np = &(self.head);
        while np.is_some() {
            list.push(np.as_ref().unwrap().data.clone());
            np = &(np.as_ref().unwrap().next);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for elem in iter.into_iter() {
            list.head = Some(Box::new(Node {
                data: elem,
                next: list.head,
            }));
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut current_node = self.head;
        while let Some(inner_current_node) = current_node {
            vec.insert(0, inner_current_node.data);
            current_node = inner_current_node.next;
        }
        vec
    }
}
