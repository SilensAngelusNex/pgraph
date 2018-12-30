use super::*;

mod panics;

#[test]
fn test_add_vertices() {
    let (v, g) = create_vertices();
    let gen = g.generation();

    for (i, id) in v.into_iter().enumerate() {
        assert_eq!(g[(id,)], i + 1);
        assert_eq!(gen, id.generation());
    }
}

#[test]
fn test_get() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();
    add_edges(&a_ids, &mut a);

    for id in a_ids {
        assert!(a.vertex_data(id).is_some());
        for (sink, _) in a.outbound_edges(id) {
            assert!(a.weight(id, *sink).is_some());
        }
    }

    for id in b_ids {
        assert!(a.vertex_data(id).is_none());
        assert!(a.outbound_edges(id).next().is_none());
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
            let calc_weight = source.data() * 10 + g[(*sink,)];
            assert_eq!(weight, &calc_weight);
        }
    }

    for source in g.ids() {
        for sink in g.ids() {
            if g.has_edge(*source, *sink) {
                let calc_weight = g[(*source,)] * 10 + g[*sink].data();
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
    *g.vertex_data_mut(v_id).unwrap() *= 2;
    *g[v_id].data_mut() *= 2;

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
    let old_v = g.vertex(v_id);
    let new_v = g.vertex(new_id);

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

        assert_eq!(a_v.data(), b_v.data());
        for (a_sink, b_sink) in a.ids().zip(b.ids()) {
            assert_eq!(a_v.weight(*a_sink), b_v.weight(*b_sink))
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
    *a.weight_mut(a_ids[0], a_ids[1]).unwrap() *= 2;
    assert_eq!(b[(b_ids[0], b_ids[1])] * 4, a[(a_ids[0], a_ids[1])]);
    assert!(a.weight_mut(a_ids[0], a_ids[0]).is_none());

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
    let mut a = PGraph::<usize, usize>::new();

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
    let before_gen = a.generation();
    a.remove_mut(a_ids[0]);
    a.disconnect_mut(a_ids[3], a_ids[1]);
    let after_gen = a.generation();

    let result = format!("{:?}", a);
    let expected = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n",
        format!("PGraph (gen {}) {{", after_gen),
        format!("\tBlank"),
        format!("\t2 (1 gen{}) => (2 gen{}: 23)", before_gen, before_gen),
        format!(
            "\t3 (2 gen{}) => (1 gen{}: 32, 3 gen{}: 34)",
            before_gen, before_gen, before_gen
        ),
        format!("\t4 (3 gen{}) => (None)", before_gen),
        "}"
    );

    assert_eq!(result, expected);
}

#[test]
fn test_edge_from_vertex() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();
    add_edges(&a_ids, &mut a);

    let v0 = a.vertex_mut(a_ids[0]).unwrap();
    assert_eq!(v0[a_ids[1]], 12);

    assert!(v0.weight(b_ids[1]).is_none());
    assert!(v0.weight_mut(b_ids[1]).is_none());
    assert!(v0.weight_mut(a_ids[1]).is_some());

    assert!(v0.disconnect(a_ids[1]));
    assert!(!v0.disconnect(b_ids[1]));
}

fn create_vertices() -> (Vec<Id>, PGraph<usize, usize>) {
    let mut graph = PGraph::default();
    let mut vec = Vec::new();

    vec.push(graph.add_mut(1));
    vec.push(graph.add_mut(2));
    vec.push(graph.add_mut(3));
    vec.push(graph.add_mut(4));

    (vec, graph)
}

fn add_edges(v: &[Id], graph: &mut PGraph<usize, usize>) {
    graph.connect_mut(v[0], v[1], 12);
    graph.connect_mut(v[1], v[2], 23);
    graph.connect_mut(v[2], v[1], 32);

    let a = graph.try_connect_mut(v[2], v[3], 34);
    let b = graph.try_connect_mut(v[3], v[1], 42);

    assert!(a && b);
}
