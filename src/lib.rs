// Copyright 2018 Weston Carvalho
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! `PGraph is a functional-style graph data structure. It uses `im` to provide stuctural sharing, which makes cloning a
//! PGraph an O(1) operation, at the cost of slighlly more expensive accesses and modifications. Methods that modify the
//! Graph clone the graph by default and return the new, modified version. There are also methods that modify the graph in-place.

mod id;
mod pgraph;

pub use crate::id::Id;
pub use crate::pgraph::{Edge, PGraph, Vertex};

#[cfg(test)]
mod tests;
