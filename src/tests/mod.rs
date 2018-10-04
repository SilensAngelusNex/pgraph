use super::*;

#[test]
fn test_add() {
    let mut g = Graph::new();
    let v1 = g.add_mut(1);
    let v2 = g.add_mut(2);
    let v3 = g.add_mut(3);

    g.connect_mut(&v1, &v2, 12);
    g.connect_mut(&v2, &v3, 23);

    let mut h = g.connect(&v3, &v2, 32).unwrap();

    println!("{:?}", g);
    println!("----------------\n\n\n");
    println!("{:?}", h);
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

    let mut h = g.connect(&v3, &v2, 32).unwrap();

    println!("{:?}", g);
    println!("----------------\n\n\n");
    println!("{:?}", h);
}
