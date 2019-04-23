use super::*;

#[test]
fn allocate() {
    let mut list = List::default();
    let a = list.allocate(1u32);

    assert_eq!(list.count, 1);
    assert_eq!(list.next_free_id, a + 1);

    let b = list.allocate(1);

    assert_eq!(list.count, 2);
    assert_eq!(list.next_free_id, b + 1);

    let c = list.allocate(1);

    assert_eq!(list.count, 3);
    assert_eq!(list.next_free_id, c + 1);


    list.free(b);

    assert_eq!(list.count, 2);
    assert_eq!(list.next_free_id, b);

    let new_id = list.allocate(1);

    assert_eq!(new_id, b);
    assert_eq!(list.get(new_id), &1);
    assert_eq!(list.count, 3);
    assert_eq!(list.next_free_id, c + 1);
}

#[test]
fn free() {
    let mut list = List::default();
    let (_a, b, _c) = (list.allocate(0), list.allocate(0), list.allocate(0));

    {
        *list.get_mut(b) = 111u32;
    }

    list.free(b);

    assert_eq!(list.count, 2);
    assert_eq!(list.next_free_id, b);
}

#[test]
fn iter() {
    let mut list = List::default();
    let (a, b, c) = (list.allocate(1u32), list.allocate(2), list.allocate(3));

    {
        let mut iter = list.iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((b, &2)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let mut iter = list.iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((b, &mut 2)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
        assert_eq!(None, iter.next());
    }

    list.free(b);

    {
        let mut iter = list.iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let mut iter = list.iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
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

    {
        let mut iter = list.iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((d, &mut 4)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
