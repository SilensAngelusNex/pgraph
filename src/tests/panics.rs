use super::*;

#[test]
#[should_panic]
fn test_get1() {
    let (_, a) = create_vertices();
    let (b_ids, _) = create_vertices();

    assert!(a.vertex(b_ids[0]).is_none());
    &a[b_ids[0]];
}

#[test]
#[should_panic]
fn test_get2() {
    let (ids, mut g) = create_vertices();
    g.remove_mut(ids[0]);

    assert!(g.vertex(ids[0]).is_none());
    &g[ids[0]];
}

#[test]
#[should_panic]
fn test_get3() {
    let (_, a) = create_vertices();
    let (_, id) = a.add(5);

    assert!(a.vertex(id).is_none());
    &a[id];
}

#[test]
#[should_panic]
fn test_get_mut1() {
    let (_, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    assert!(a.vertex_mut(b_ids[0]).is_none());
    &mut a[b_ids[0]];
}

#[test]
#[should_panic]
fn test_get_mut2() {
    let (ids, mut g) = create_vertices();
    g.remove_mut(ids[0]);

    assert!(g.vertex_mut(ids[0]).is_none());
    &mut g[ids[0]];
}

#[test]
#[should_panic]
fn test_get_mut3() {
    let (_, mut a) = create_vertices();
    let (_, id) = a.add(5);

    assert!(a.vertex_mut(id).is_none());
    &mut a[id];
}

#[test]
#[should_panic]
fn test_connect1() {
    let (a_ids, a) = create_vertices();
    let (b_ids, _) = create_vertices();

    let _ = a.connect(a_ids[1], b_ids[2], 4);
}

#[test]
#[should_panic]
fn test_connect2() {
    let (a_ids, a) = create_vertices();
    let (b_ids, _) = create_vertices();

    let _ = a.connect(b_ids[1], a_ids[2], 4);
}

#[test]
#[should_panic]
fn test_connect_mut1() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    let _ = a.connect_mut(a_ids[1], b_ids[2], 4);
}

#[test]
#[should_panic]
fn test_connect_mut2() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    a.connect_mut(b_ids[1], a_ids[2], 4);
}
