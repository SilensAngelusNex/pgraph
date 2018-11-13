[![Build Status](https://travis-ci.com/SilensAngelusNex/pgraph.svg?branch=master)](https://travis-ci.com/SilensAngelusNex/pgraph) [![codecov](https://codecov.io/gh/SilensAngelusNex/pgraph/branch/master/graph/badge.svg)](https://codecov.io/gh/SilensAngelusNex/pgraph)

# PGraph

PGraph provides a persistent graph data structure with the following features:
* Supports optional data for vertices and edges (For example, `Graph<(), E>` has data on edges only.)
* Directed edges, for that sweet, sweet versitility
* Structural sharing built on [rpds](https://crates.io/crates/rpds) data structures
* TODO: Common graph algorithms (Dijkstra's, Bellman-Ford, and Floyd-Warshall are the first ones I'm planning, but make an issue if there's another that you want.)

## Setup

To use pgraph add the following to your `Cargo.toml`:

```toml
[dependencies]
pgraph = { git = "https://github.com/SilensAngelusNex/pgraph.git" }
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

let mut h = g.remove(&v1);
let v6 = h.add_mut(6);
h.connect_mut(&v6, &v5, 65);
*h.get_edge_mut(&v5, &v3).unwrap() = 130;

let i = h.clone();

let mut j = i.remove(&v4);
let v7 = j.add_mut(7);
j.connect_mut(&v6, &v7, 67);

println!("g = {:?}", g);
println!("----------------\n\n");
println!("h = {:?}", h);
println!("----------------\n\n");
println!("i = {:?}", i);
println!("----------------\n\n");
println!("j = {:?}", j);
println!("----------------\n\n");
println!("_ = {:?}", Graph::<i8, i8>::new());
```

Output:
```
g = Graph (gen 0) {
	1 (0 gen0) => (1 gen0: 12)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32, 3 gen0: 34)
	4 (3 gen0) => (0 gen0: 41)
	5 (4 gen0) => (2 gen0: 53)
}

----------------


h = Graph (gen 1) {
	6 (0 gen1) => (4 gen0: 65)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32, 3 gen0: 34)
	4 (3 gen0) => (None)
	5 (4 gen0) => (2 gen0: 130)
}

----------------


i = Graph (gen 2) {
	6 (0 gen1) => (4 gen0: 65)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32, 3 gen0: 34)
	4 (3 gen0) => (None)
	5 (4 gen0) => (2 gen0: 130)
}

----------------


j = Graph (gen 3) {
	6 (0 gen1) => (3 gen3: 67, 4 gen0: 65)
	2 (1 gen0) => (2 gen0: 23)
	3 (2 gen0) => (1 gen0: 32)
	7 (3 gen3) => (None)
	5 (4 gen0) => (2 gen0: 130)
}

----------------


_ = Graph (gen 4) {}
```
