use std;

pub type Id = usize;

pub trait Nullable where Self: Copy {
    const NULL: Self;
}

impl Nullable for Id {
    const NULL: Id = std::usize::MAX;
}

#[derive(Clone)]
pub struct Node<T> {
    pub data: Option<T>,
    pub prev: Id,
    pub next: Id,
}

impl<T> Node<T> {
    pub fn new(data: T, prev: Id, next: Id) -> Node<T> {
        Node {
            data: Some(data),
            prev,
            next,
        }
    }

    pub fn empty(prev: Id, next: Id) -> Node<T> {
        Node {
            data: None,
            prev,
            next,
        }
    }
}