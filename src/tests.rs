use super::*;

#[test]
fn push_back() {
    let mut list = List::default();
    let a = list.push_back(1u32);

    assert_eq!(1, list.count);
    assert_eq!(a, list.tail_id);

    let b = list.push_back(1);

    assert_eq!(2, list.count);
    assert_eq!(b, list.tail_id);

    let c = list.push_back(1);

    assert_eq!(3, list.count);
    assert_eq!(c, list.tail_id);
}

#[test]
fn pop() {
    let mut list = List::default();
    let (_a, b, c) = (list.push_back(0), list.push_back(0), list.push_back(0));

    {
        *list.get_mut(b) = 111u32;
    }

    list.pop(b);

    assert_eq!(2, list.count);
    assert_eq!(c, list.tail_id);
    assert_eq!(b, list.nodes[list.tail_id].next);
}

#[test]
fn push_pop() {
    let mut list = List::default();

    let (_a, b, _c) = (list.push_back(1u32), list.push_back(1), list.push_back(1));

    list.pop(b);

    let new_id = list.push_back(1);

    assert_eq!(b, new_id);
    assert_eq!(&1, list.get(new_id));
    assert_eq!(3, list.count);
    assert_eq!(new_id, list.tail_id);
}

#[test]
fn unordered_iter() {
    let mut list = List::default();
    let (a, b, c) = (list.push_back(1u32), list.push_back(2), list.push_back(3));

    {
        let mut iter = list.unordered_iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((b, &2)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let mut iter = list.unordered_iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((b, &mut 2)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
        assert_eq!(None, iter.next());
    }

    list.pop(b);

    {
        let mut iter = list.unordered_iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let mut iter = list.unordered_iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
        assert_eq!(None, iter.next());
    }

    let d = list.push_back(4);

    {
        let mut iter = list.unordered_iter();

        assert_eq!(Some((a, &1)), iter.next());
        assert_eq!(Some((d, &4)), iter.next());
        assert_eq!(Some((c, &3)), iter.next());
        assert_eq!(None, iter.next());
    }

    {
        let mut iter = list.unordered_iter_mut();

        assert_eq!(Some((a, &mut 1)), iter.next());
        assert_eq!(Some((d, &mut 4)), iter.next());
        assert_eq!(Some((c, &mut 3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
