
pub struct FreeListNode<T> {
    pub data: Option<T>,
    pub next: usize,
}

impl<T> FreeListNode<T> {
    pub fn new(data: T, next: usize) -> FreeListNode<T> {
        FreeListNode {
            data: Some(data),
            next,
        }
    }

    pub fn empty(next: usize) -> FreeListNode<T> {
        FreeListNode {
            data: None,
            next,
        }
    }
}