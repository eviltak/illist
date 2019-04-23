
pub struct Node<T> {
    pub data: Option<T>,
    pub next: usize,
}

impl<T> Node<T> {
    pub fn new(data: T, next: usize) -> Node<T> {
        Node {
            data: Some(data),
            next,
        }
    }

    pub fn empty(next: usize) -> Node<T> {
        Node {
            data: None,
            next,
        }
    }
}