use super::*;

mod panics;

#[test]
fn test_add_vertices() {
    let (v, g) = create_vertices();
    let gen = g.get_gen();

    for (i, id) in v.into_iter().enumerate() {
        assert_eq!(g[(id,)], i + 1);
        assert_eq!(gen, id.get_gen());
    }
}

#[test]
fn test_get() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();
    add_edges(&a_ids, &mut a);

    for id in a_ids {
        assert!(a.get_data(id).is_some());
        for (sink, _) in a.neighbors(id) {
            assert!(a.get_edge(id, *sink).is_some());
        }
    }

    for id in b_ids {
        assert!(a.get_data(id).is_none());
        assert!(a.neighbors(id).next().is_none());
    }
}

#[test]
fn test_connect_mut() {
    let (ids, mut g) = create_vertices();
    add_edges(&ids, &mut g);

    assert!(g.has_edge(ids[0], ids[1]));
    assert!(g.has_edge(ids[1], ids[2]));
    assert!(g.has_edge(ids[2], ids[1]));
    assert!(g.has_edge(ids[2], ids[3]));
    assert!(g.has_edge(ids[3], ids[1]));

    for source in &g {
        for (sink, weight) in source {
            let calc_weight = source.get_data() * 10 + g[(*sink,)];
            assert_eq!(weight, &calc_weight);
        }
    }

    for source in g.ids() {
        for sink in g.ids() {
            if g.has_edge(*source, *sink) {
                let calc_weight = g[(*source,)] * 10 + g[*sink].get_data();
                assert_eq!(g[(*source, *sink)], calc_weight);
            }
        }
    }
}

#[test]
fn test_get_mut() {
    let v = 2; // Vertex to check
    let (ids, mut g) = create_vertices();
    let v_id = ids[v - 1];

    g[(v_id,)] *= 2;
    *g.get_data_mut(v_id).unwrap() *= 2;
    *g[v_id].get_data_mut() *= 2;

    assert_eq!(g[(v_id,)], v * 8);
}

#[test]
fn test_remove() {
    let v = 2; // Vertex to check
    let (ids, mut g) = create_vertices();
    add_edges(&ids, &mut g);
    let v_id = ids[v - 1];

    assert!(g.remove_mut(v_id));
    assert!(!g.remove_mut(v_id));

    assert_eq!(g.ids().count() + 1, ids.len());

    assert!(g.iter_data().fold(true, |result, w| result && w != &v));
    assert!(g
        .iter_weights()
        .fold(true, |result, w| result && w % 10 != v && w / 10 != v));

    assert!(g.ids().count() < ids.iter().count());

    let new_id = g.add_mut(6);
    let old_v = g.get(v_id);
    let new_v = g.get(new_id);

    assert!(old_v.is_none());
    assert!(new_v.is_some());
}

#[test]
fn test_connect() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    let c = a.connect(a_ids[1], a_ids[0], 6);
    assert!(c.has_edge(a_ids[1], a_ids[0]));
    assert_eq!(c[(a_ids[1], a_ids[0])], 6);

    let c_opt = a.try_connect(a_ids[1], a_ids[0], 6);
    assert!(c_opt.is_some());
    let c = c_opt.unwrap();
    assert!(c.has_edge(a_ids[1], a_ids[0]));
    assert_eq!(c[(a_ids[1], a_ids[0])], 6);

    let c_opt = a.try_connect(a_ids[1], b_ids[0], 6);
    assert!(c_opt.is_none());

    let c_opt = a.try_connect(b_ids[1], a_ids[0], 6);
    assert!(c_opt.is_none());

    assert!(!a.try_connect_mut(a_ids[1], b_ids[0], 6));
    assert!(!a.try_connect_mut(b_ids[1], a_ids[0], 6));

    assert!(a.try_connect_mut(a_ids[1], a_ids[0], 6));
    assert!(a.has_edge(a_ids[1], a_ids[0]));
    assert_eq!(a[(a_ids[1], a_ids[0])], 6);
}

#[test]
fn test_recreate() {
    let (a_ids, mut a) = create_vertices();
    add_edges(&a_ids, &mut a);

    let a = a.try_remove(a_ids[2]).unwrap();
    let mut a = a.remove(a_ids[3]);
    let added = a.add_mut(5);

    a.remove_mut(a_ids[1]);
    a.connect_mut(added, a_ids[0], 51);
    a.connect_mut(a_ids[0], added, 15);

    let b = a.recreate();

    for (a_id, b_id) in a.ids().zip(b.ids()) {
        let a_v = &a[*a_id];
        let b_v = &b[*b_id];

        assert_eq!(a_v.get_data(), b_v.get_data());
        for (a_sink, b_sink) in a.ids().zip(b.ids()) {
            assert_eq!(a_v.get_cost(*a_sink), b_v.get_cost(*b_sink))
        }
    }

    assert_ne!(a.count_empties(), 0);
    assert_eq!(b.count_empties(), 0);
}

#[test]
fn test_edges() {
    let (a_ids, mut a) = create_vertices();
    add_edges(&a_ids, &mut a);

    let (b_ids, mut b) = create_vertices();
    add_edges(&b_ids, &mut b);

    let mut a = a.remove(b_ids[0]);
    assert!(a.try_remove(b_ids[0]).is_none());

    a[(a_ids[0], a_ids[1])] *= 2;
    *a.get_edge_mut(a_ids[0], a_ids[1]).unwrap() *= 2;
    assert_eq!(b[(b_ids[0], b_ids[1])] * 4, a[(a_ids[0], a_ids[1])]);
    assert!(a.get_edge_mut(a_ids[0], a_ids[0]).is_none());

    let c = b.disconnect(b_ids[2], b_ids[3]);
    assert!(b.has_edge(b_ids[2], b_ids[3]));
    assert!(!c.has_edge(b_ids[2], b_ids[3]));

    assert!(b.try_disconnect(a_ids[2], a_ids[3]).is_none());
    assert!(b.try_disconnect(a_ids[2], b_ids[3]).is_none());
    assert!(b.try_disconnect(b_ids[2], a_ids[3]).is_none());
    assert!(b.try_disconnect(b_ids[2], b_ids[3]).is_some());

    assert!(!b.try_disconnect_mut(a_ids[2], a_ids[3]));
    assert!(!b.try_disconnect_mut(b_ids[2], a_ids[3]));
    assert!(!b.try_disconnect_mut(a_ids[2], b_ids[3]));
    assert!(b.try_disconnect_mut(b_ids[2], b_ids[3]));
    assert!(!b.has_edge(b_ids[2], b_ids[3]));
}

#[test]
fn test_remove_all() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    let a_count = a.into_iter().count();

    let c = a.remove_all(&a_ids);
    assert_eq!(c.into_iter().count(), 0);

    let d = a.try_remove_all(&a_ids);
    assert!(d.is_some());
    let e = a.try_remove_all(&b_ids);
    assert!(e.is_none());

    a.remove_all_mut(&b_ids);
    assert_eq!(a.into_iter().count(), a_count);

    a.remove_all_mut(&a_ids);
    assert_eq!(a.into_iter().count(), 0);
}

#[test]
fn test_add_all() {
    let vertices: Vec<usize> = vec![0, 1, 2, 3, 4];
    let mut a = Graph::<usize, usize>::new();

    let (b, b_ids) = a.add_all(vertices.clone());
    let a_ids = a.add_all_mut(vertices);

    for (i, (a_id, b_id)) in a_ids.into_iter().zip(b_ids).enumerate() {
        assert_eq!(a[(a_id,)], i);
        assert_eq!(b[(b_id,)], i);
    }
}

#[test]
fn test_debug() {
    let (a_ids, mut a) = create_vertices();
    add_edges(&a_ids, &mut a);
    let gen = a.get_gen();

    let result = format!("{:?}", a);
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n",
        format!("Graph (gen {}) {{", gen),
        format!("\t1 (0 gen{}) => (1 gen{}: 12)", gen, gen),
        format!("\t2 (1 gen{}) => (2 gen{}: 23)", gen, gen),
        format!("\t3 (2 gen{}) => (1 gen{}: 32, 3 gen{}: 34)", gen, gen, gen),
        format!("\t4 (3 gen{}) => (1 gen{}: 42)", gen, gen),
        "}"
    );

    assert_eq!(result, expected);
}

fn create_vertices() -> (Vec<Id>, Graph<usize, usize>) {
    let mut graph = Graph::default();
    let mut vec = Vec::new();

    vec.push(graph.add_mut(1));
    vec.push(graph.add_mut(2));
    vec.push(graph.add_mut(3));
    vec.push(graph.add_mut(4));

    (vec, graph)
}

fn add_edges(v: &[Id], graph: &mut Graph<usize, usize>) {
    graph.connect_mut(v[0], v[1], 12);
    graph.connect_mut(v[1], v[2], 23);
    graph.connect_mut(v[2], v[1], 32);

    let a = graph.try_connect_mut(v[2], v[3], 34);
    let b = graph.try_connect_mut(v[3], v[1], 42);

    assert!(a && b);
}
