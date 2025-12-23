use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node(pub usize);

impl Add<usize> for Node {
    type Output = Node;

    fn add(self, rhs: usize) -> Self::Output {
        Node(self.0 + rhs)
    }
}

impl Sub<usize> for Node {
    type Output = Node;

    fn sub(self, rhs: usize) -> Self::Output {
        Node(self.0 - rhs)
    }
}
