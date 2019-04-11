#[cfg(test)]
mod tests;

pub struct FreeListNode<T> {
    data: Option<T>,
    next: FreeListId,
}

impl<T> FreeListNode<T> {
    pub fn new(data: T, next: FreeListId) -> FreeListNode<T> {
        FreeListNode {
            data: Some(data),
            next,
        }
    }

    pub fn empty(next: FreeListId) -> FreeListNode<T> {
        FreeListNode {
            data: None,
            next,
        }
    }
}

pub type FreeListId = usize;

pub struct FreeList<T> {
    nodes: Vec<FreeListNode<T>>,
    count: usize,
    next_free_id: FreeListId,
}

impl<T> FreeList<T> {
    pub fn new(initial_capacity: usize) -> FreeList<T> {
        let mut node_pool = Vec::with_capacity(initial_capacity);

        FreeList {
            nodes: node_pool,
            count: 0,
            next_free_id: 0,
        }
    }

    pub fn allocate(&mut self, data: T) -> FreeListId {
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

    pub fn free(&mut self, id: FreeListId) {
        let pool_object = self.nodes.get_mut(id).expect("Invalid object id");

        pool_object.next = self.next_free_id;
        pool_object.data = None;

        self.next_free_id = id;

        self.count -= 1;
    }

    pub fn get(&self, id: FreeListId) -> &T {
        self.nodes.get(id)
            .and_then(|node| node.data.as_ref())
            .expect("Invalid object id")
    }

    pub fn get_mut(&mut self, id: FreeListId) -> &mut T {
        self.nodes.get_mut(id)
            .and_then(|node| node.data.as_mut())
            .expect("Invalid object id")
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn iter(&self) -> impl Iterator<Item = (FreeListId, &T)> {
        return self.nodes.iter()
            .enumerate()
            .filter_map(|(id, node)| {
                node.data.as_ref().map(|object| (id, object))
            });
    }
}

impl<T> Default for FreeList<T> {
    fn default() -> FreeList<T> {
        FreeList::new(64)
    }
}
