#![cfg(test)]

use super::*;

#[test]
fn allocate() {
    let mut pool = FreeList::default();
    let id_a = pool.allocate(0);
    {
        let a = pool.get_mut(id_a);
        *a = 32u32;
    }
    assert_eq!(pool.object_count, 1);
    assert_eq!(pool.next_free_object_id, id_a + 1);
    assert_eq!(pool.get(id_a), &32);
}

#[test]
fn free() {
    let mut pool = FreeList::default();
    let (a, b, c) = (pool.allocate(0), pool.allocate(0), pool.allocate(0));
    
    {
        *pool.get_mut(b) = 111u32;
    }
    
    pool.free(b);
    
    assert_eq!(pool.object_count, 2);
    assert_eq!(pool.next_free_object_id, b);
    
    let new_id = pool.allocate(0);
    
    assert_eq!(b, new_id);
    assert_eq!(pool.get(new_id), &u32::default())
}
