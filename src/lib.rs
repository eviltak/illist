mod node;

#[cfg(test)]
mod tests;

use node::FreeListNode;

pub struct FreeList<T> {
    nodes: Vec<FreeListNode<T>>,
    count: usize,
    next_free_id: usize,
}

impl<T> FreeList<T> {
    pub fn new(initial_capacity: usize) -> FreeList<T> {
        let nodes = Vec::with_capacity(initial_capacity);

        FreeList {
            nodes,
            count: 0,
            next_free_id: 0,
        }
    }

    pub fn allocate(&mut self, data: T) -> usize {
        debug_assert!(self.next_free_id <= self.nodes.len());

        if self.next_free_id == self.nodes.len() {
            // Expand our node pool
            self.nodes.push(FreeListNode::new(data, self.next_free_id + 1));
        } else {
            self.nodes[self.next_free_id].data = Some(data);
        }

        self.count += 1;

        let object_id = self.next_free_id;

        self.next_free_id = self.nodes[object_id].next;

        object_id
    }

    pub fn free(&mut self, id: usize) {
        let pool_object = self.nodes.get_mut(id).expect("Invalid object id");

        pool_object.next = self.next_free_id;
        pool_object.data = None;

        self.next_free_id = id;

        self.count -= 1;
    }

    pub fn get(&self, id: usize) -> &T {
        self.nodes.get(id)
            .and_then(|node| node.data.as_ref())
            .expect("Invalid object id")
    }

    pub fn get_mut(&mut self, id: usize) -> &mut T {
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

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.nodes.iter()
            .enumerate()
            .filter_map(|(id, node)| {
                node.data.as_ref().map(|object| (id, object))
            })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.nodes.iter_mut()
            .enumerate()
            .filter_map(|(id, node)| {
                node.data.as_mut().map(|object| (id, object))
            })
    }
}

impl<T> Default for FreeList<T> {
    fn default() -> FreeList<T> {
        FreeList::new(64)
    }
}
