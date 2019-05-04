mod node;

#[cfg(test)]
mod tests;

pub use node::Id;
use node::{Node, Nullable};

#[derive(Default, Clone)]
pub struct List<T> {
    nodes: Vec<Node<T>>,
    count: usize,
    tail_id: Id,
}

impl<T> List<T> {
    pub fn new(initial_capacity: usize) -> List<T> {
        let nodes = Vec::with_capacity(initial_capacity);

        List {
            nodes,
            count: 0,
            tail_id: 0,
        }
    }

    pub fn push_back(&mut self, data: T) -> usize {
        let (new_id, prev_id) =
            match self.nodes.get(self.tail_id) {
                Some(node) => {
                    (node.next, self.tail_id)
                }
                None => {
                    (self.tail_id, Id::NULL)
                }
            };

        match self.nodes.get_mut(new_id) {
            Some(node) => {
                node.prev = prev_id;
                node.next = Id::NULL;
                node.data = Some(data);

                self.tail_id = new_id;
            }
            None => {
                self.nodes.push(Node::new(data, prev_id, Id::NULL));

                self.tail_id = self.nodes.len() - 1
            }
        }

        if let Some(prev) = self.nodes.get_mut(prev_id) {
            prev.next = self.tail_id;
        }

        self.count += 1;

        self.tail_id
    }

    pub fn pop(&mut self, id: Id) {
        // TODO: Use indexing
        let pool_object = self.nodes.get_mut(id).expect("Invalid object id");

        pool_object.next = Id::NULL;
        pool_object.data = None;

        self.nodes[self.tail_id].next = id;

        self.count -= 1;
    }

    pub fn get(&self, id: Id) -> &T {
        self.nodes.get(id)
            .and_then(|node| node.data.as_ref())
            .expect("Invalid object id")
    }

    pub fn get_mut(&mut self, id: Id) -> &mut T {
        self.nodes.get_mut(id)
            .and_then(|node| node.data.as_mut())
            .expect("Invalid object id")
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn unordered_iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.nodes.iter()
            .enumerate()
            .filter_map(|(id, node)| {
                node.data.as_ref().map(|object| (id, object))
            })
    }

    pub fn unordered_iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        self.nodes.iter_mut()
            .enumerate()
            .filter_map(|(id, node)| {
                node.data.as_mut().map(|object| (id, object))
            })
    }
}
