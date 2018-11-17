// Copyright 2018 Weston Carvalho
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rpds;
#[cfg(test)]
#[macro_use]
extern crate more_asserts;
mod graph;
mod id;

pub use graph::vertex::adj::Edge;
pub use graph::vertex::Vertex;
pub use graph::Graph;
pub use id::Id;

#[cfg(test)]
mod tests;
