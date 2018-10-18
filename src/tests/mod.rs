use super::*;

#[test]
fn test_add_vertices() {
    let (v, g) = create_vertices();
    let gen = g.get_gen();

    for (i, id) in v.into_iter().enumerate() {
        assert_eq!(*g.get_data(&id), i + 1);
        assert_eq!(gen, id.get_gen());
    }
}

#[test]
fn test_try_get() {
    let (a_ids, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();
    add_edges(&a_ids, &mut a);

    for id in a_ids {
        assert!(a.try_get_data(&id).is_some());
        for (sink, _) in a.neighbors(&id) {
            assert!(a.try_get_edge(&id, &sink).is_some());
        }
    }

    for id in b_ids {
        assert!(a.try_get_data(&id).is_none());
        assert!(a.neighbors(&id).next().is_none());
    }
}

#[test]
fn test_connect() {
    let (ids, mut g) = create_vertices();
    add_edges(&ids, &mut g);

    assert!(g.has_edge(&ids[0], &ids[1]));
    assert!(g.has_edge(&ids[1], &ids[2]));
    assert!(g.has_edge(&ids[2], &ids[1]));
    assert!(g.has_edge(&ids[2], &ids[3]));
    assert!(g.has_edge(&ids[3], &ids[1]));

    for source in &g {
        for (sink, weight) in source {
            let calc_weight = source.get_data() * 10 + g[(sink,)];
            assert_eq!(weight, &calc_weight);
        }
    }

    for source in g.ids() {
        for sink in g.ids() {
            if g.has_edge(source, sink) {
                let calc_weight = g.get_data(source) * 10 + g[sink].get_data();
                assert_eq!(g[(source, sink)], calc_weight);
            }
        }
    }
}

#[test]
#[should_panic]
fn test_get_panic1() {
    let (_, a) = create_vertices();
    let (b_ids, _) = create_vertices();

    assert!(a.try_get(&b_ids[0]).is_none());
    a.get(&b_ids[0]);
}

#[test]
#[should_panic]
fn test_get_panic2() {
    let (ids, mut g) = create_vertices();
    g.remove_mut(&ids[0]);

    assert!(g.try_get(&ids[0]).is_none());
    g.get(&ids[0]);
}

#[test]
#[should_panic]
fn test_get_panic3() {
    let (_, a) = create_vertices();
    let (_, id) = a.add(5);

    assert!(a.try_get(&id).is_none());
    a.get(&id);
}

#[test]
#[should_panic]
fn test_get_mut_panic1() {
    let (_, mut a) = create_vertices();
    let (b_ids, _) = create_vertices();

    assert!(a.try_get_mut(&b_ids[0]).is_none());
    a.get_mut(&b_ids[0]);
}

#[test]
#[should_panic]
fn test_get_mut_panic2() {
    let (ids, mut g) = create_vertices();
    g.remove_mut(&ids[0]);

    assert!(g.try_get_mut(&ids[0]).is_none());
    g.get_mut(&ids[0]);
}

#[test]
#[should_panic]
fn test_get_mut_panic3() {
    let (_, mut a) = create_vertices();
    let (_, id) = a.add(5);

    assert!(a.try_get_mut(&id).is_none());
    a.get_mut(&id);
}

#[test]
fn test_try_get_mut() {
    let v = 2; // Vertex to check
    let (ids, mut g) = create_vertices();
    let v_id = &ids[v - 1];

    let mut x = 5;
    x *= 4;
}

#[test]
fn test_remove() {
    let v = 2; // Vertex to check
    let (ids, mut g) = create_vertices();
    add_edges(&ids, &mut g);
    let v_id = &ids[v - 1];

    g.remove_mut(v_id);
    assert!(g.ids().count() < ids.len());

    assert!(g.iter_data().fold(true, |result, w| result && w != &v));
    assert!(
        g.iter_weights()
            .fold(true, |result, w| result && w % 10 != v && w / 10 != v)
    );

    let new_id = g.add_mut(6);

    let old_v = g.try_get(v_id);
    let new_v = g.try_get(&new_id);

    assert!(old_v.is_none());
    assert!(new_v.is_some());
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
    graph.connect_mut(&v[0], &v[1], 12);
    graph.connect_mut(&v[1], &v[2], 23);
    graph.connect_mut(&v[2], &v[1], 32);
    graph.connect_mut(&v[2], &v[3], 34);
    graph.connect_mut(&v[3], &v[1], 42);
}
