mod tests;

struct FreeListNode<T> {
    data: T,
    next: FreeListId,
}

impl<T> FreeListNode<T> {
    pub fn new(data: T, next: FreeListId) -> FreeListNode<T> {
        FreeListNode {
            data,
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
        }
        else {
            self.object_pool[self.next_free_object_id].data = data;
        }
        
        self.object_count += 1;
        
        let object_id = self.next_free_object_id;
        self.next_free_object_id += 1;
        object_id
    }
    
    pub fn free(&mut self, id: FreeListId) {
        let pool_object = self.object_pool.get_mut(id).expect("Invalid object id");
        pool_object.next = self.next_free_object_id;
        self.next_free_object_id = id;
        
        self.object_count -= 1;
    }
    
    pub fn get(&self, id: FreeListId) -> &T {
        &self.object_pool.get(id).expect("Invalid object id").data
    }
    
    pub fn get_mut(&mut self, id: FreeListId) -> &mut T {
        &mut self.object_pool.get_mut(id).expect("Invalid object id").data
    }
}

impl<T> Default for FreeList<T> {
    fn default() -> FreeList<T> {
        FreeList::new(64)
    }
}
