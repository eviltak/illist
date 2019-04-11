use super::*;

#[test]
fn allocate() {
    let mut pool = FreeList::default();
    let a = pool.allocate(1u32);

    assert_eq!(pool.object_count, 1);
    assert_eq!(pool.next_free_object_id, a + 1);

    let b = pool.allocate(1);

    assert_eq!(pool.object_count, 2);
    assert_eq!(pool.next_free_object_id, b + 1);

    let c = pool.allocate(1);

    assert_eq!(pool.object_count, 3);
    assert_eq!(pool.next_free_object_id, c + 1);


    pool.free(b);

    assert_eq!(pool.object_count, 2);
    assert_eq!(pool.next_free_object_id, b);

    let new_id = pool.allocate(1);

    assert_eq!(new_id, b);
    assert_eq!(pool.get(new_id), &1);
    assert_eq!(pool.object_count, 3);
    assert_eq!(pool.next_free_object_id, c + 1);
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
}

#[test]
fn iter() {
    let mut list = FreeList::default();
    let (a, b, c) = (list.allocate(1u32), list.allocate(2), list.allocate(3));

    {
        let mut iter = list.iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((b, &2)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    list.free(b);

    {
        let mut iter = list.iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    let d = list.allocate(4);

    {
        let mut iter = list.iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((d, &4)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
