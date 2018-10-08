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
            let calc_weight = source.get_data() * 10 + g.get_data(sink);
            assert_eq!(weight, &calc_weight);
        }
    }

    for source in g.ids() {
        for sink in g.ids() {
            if g.has_edge(source, sink) {
                let calc_weight = g.get_data(&source) * 10 + g.get_data(&sink);
                assert_eq!(g.get_edge(source, sink), &calc_weight);
            }
        }
    }
}

#[test]
fn test_remove() {
    let mut g = Graph::new();
    let v1 = g.add_mut(1);
    let v2 = g.add_mut(2);
    let v3 = g.add_mut(3);
    let v4 = g.add_mut(4);
    let v5 = g.add_mut(5);

    g.connect_mut(&v1, &v2, 12);
    g.connect_mut(&v2, &v3, 23);
    g.connect_mut(&v3, &v2, 32);
    g.connect_mut(&v4, &v1, 41);
    g.connect_mut(&v5, &v3, 53);
    g.connect_mut(&v3, &v4, 34);

    let mut h = g.connect(&v3, &v2, 32);

    println!("{:?}", g);
    println!("----------------\n\n\n");
    println!("{:?}", h);
}

fn create_vertices() -> (Vec<Id>, Graph<usize, usize>) {
    let mut graph = Graph::new();
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
