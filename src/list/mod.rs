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
    object_pool: Vec<FreeListNode<T>>,
    pub object_count: usize,
    next_free_object_id: FreeListId,
}

impl<T> FreeList<T> {
    pub fn new(initial_capacity: FreeListId) -> FreeList<T> {
        let mut node_pool = Vec::with_capacity(initial_capacity);

        FreeList {
            object_pool: node_pool,
            object_count: 0,
            next_free_object_id: 0,
        }
    }

    pub fn allocate(&mut self, data: T) -> FreeListId {
        assert!(self.next_free_object_id <= self.object_pool.len());

        if self.next_free_object_id == self.object_pool.len() {
            // Expand our node pool
            self.object_pool.push(FreeListNode::new(data, self.next_free_object_id + 1));
        } else {
            self.object_pool[self.next_free_object_id].data = Some(data);
        }

        self.object_count += 1;

        let object_id = self.next_free_object_id;

        self.next_free_object_id = self.object_pool[object_id].next;

        object_id
    }

    pub fn free(&mut self, id: FreeListId) {
        let pool_object = self.object_pool.get_mut(id).expect("Invalid object id");

        pool_object.next = self.next_free_object_id;
        pool_object.data = None;

        self.next_free_object_id = id;

        self.object_count -= 1;
    }

    pub fn get(&self, id: FreeListId) -> &T {
        self.object_pool.get(id)
            .and_then(|node| node.data.as_ref())
            .expect("Invalid object id")
    }

    pub fn get_mut(&mut self, id: FreeListId) -> &mut T {
        self.object_pool.get_mut(id)
            .and_then(|node| node.data.as_mut())
            .expect("Invalid object id")
    }

    pub fn iter(&self) -> impl Iterator<Item = (FreeListId, &T)> {
        return self.object_pool.iter()
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
