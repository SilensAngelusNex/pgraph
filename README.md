# PGraph

PGraph provides a persistent graph data structure with the following features:
* Structural sharing built on [rpds](https://crates.io/crates/rpds) data structures
* Generic over vertex values and edge weights
* TODO: Common graph algorithms (Dijkstra's, Bellman-Ford, and Floyd-Warshall are the first ones I'm planning, but make an issue if there's another that you want.)

## Setup

To use rpds add the following to your `Cargo.toml`:

```toml
[dependencies]
rpds = "<version>"
```

## Usage

```rust

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

let mut h = g.remove(&v1).unwrap();
let v6 = h.add_mut(6);
h.connect_mut(&v6, &v5, 65);

{
    let e = h.get_edge_mut(&v5, &v3).unwrap();
    *e = 153;
};

println!("{:?}", g);
println!("----------------\n\n");
println!("{:?}", h);
```

Output:
```
Graph {
	1 (0 gen0) => (1 gen0: 12)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32, 3 gen0: 34)
	4 (3 gen0) => (0 gen0: 41)
	5 (4 gen0) => (2 gen0: 53)
}

----------------


Graph {
	6 (0 gen1) => (4 gen0: 65)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32, 3 gen0: 34)
	4 (3 gen0) => (None)
	5 (4 gen0) => (2 gen0: 153)
}
```