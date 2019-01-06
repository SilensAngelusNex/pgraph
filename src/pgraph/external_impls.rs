use super::vertex::Vertex;
use super::{Id, OutboundIter, PGraph, PredecessorIter};
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::{
    Data, GraphBase, GraphProp, IntoEdgeReferences, IntoEdges, IntoEdgesDirected, IntoNeighbors,
    IntoNeighborsDirected, IntoNodeIdentifiers, NodeCompactIndexable, NodeCount, NodeIndexable,
    Visitable,
};
use petgraph::{Directed, Direction};
use std::collections::HashSet;
use std::iter::Map;

impl<V, E> GraphBase for PGraph<V, E> {
    type NodeId = Id;
    type EdgeId = (Id, Id);
}

impl<V, E> GraphProp for PGraph<V, E> {
    type EdgeType = Directed;
}

impl<V, E> Data for PGraph<V, E> {
    type NodeWeight = V;
    type EdgeWeight = E;
}

impl<V, E> Visitable for PGraph<V, E> {
    type Map = HashSet<Id>;

    fn visit_map(&self) -> Self::Map {
        HashSet::new()
    }

    fn reset_map(&self, map: &mut Self::Map) {
        map.clear()
    }
}

impl<'a, V, E> IntoNeighbors for &'a PGraph<V, E> {
    type Neighbors = <Self as IntoNeighborsDirected>::NeighborsDirected;

    fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
        self.neighbors_directed(a, Direction::Outgoing)
    }
}

pub enum NeighborIter<'a, V, E> {
    Outgoing(super::OutboundIdIter<'a, E>),
    Incoming(super::PredecessorIdIter<'a, V, E>),
}

impl<'a, V, E> NeighborIter<'a, V, E> {
    fn from(g: &'a PGraph<V, E>, id: Id, d: Direction) -> Self {
        match d {
            Direction::Outgoing => NeighborIter::Outgoing(g.outbound_ids(id)),
            Direction::Incoming => NeighborIter::Incoming(g.predecessor_ids(id)),
        }
    }
}

impl<'a, V, E> Iterator for NeighborIter<'a, V, E> {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            NeighborIter::Outgoing(iter) => iter.next(),
            NeighborIter::Incoming(iter) => iter.next(),
        }
    }
}

impl<'a, V, E> IntoNeighborsDirected for &'a PGraph<V, E> {
    type NeighborsDirected = NeighborIter<'a, V, E>;

    fn neighbors_directed(self, n: Self::NodeId, d: Direction) -> Self::NeighborsDirected {
        NeighborIter::from(self, n, d)
    }
}

impl<'a, V, E> IntoNodeIdentifiers for &'a PGraph<V, E> {
    type NodeIdentifiers = super::IdIter<'a, V, E>;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        self.ids()
    }
}

impl<'a, V, E> IntoEdgeReferences for &'a PGraph<V, E> {
    type EdgeRef = (Id, Id, &'a E);
    type EdgeReferences = super::EdgeIter<'a, V, E>;

    fn edge_references(self) -> Self::EdgeReferences {
        self.edges()
    }
}

impl<'a, V, E> IntoNodeReferences for &'a PGraph<V, E> {
    type NodeRef = (Id, &'a V);
    type NodeReferences = NodeRefIter<'a, V, E>;

    fn node_references(self) -> Self::NodeReferences {
        self.into_iter().map(|v| (v.id(), v.data()))
    }
}

type NodeRefIter<'a, V, E> = Map<super::VertexIter<'a, V, E>, fn(&'a Vertex<V, E>) -> (Id, &'a V)>;

impl<'a, V, E> IntoEdges for &'a PGraph<V, E> {
    type Edges = std::iter::Chain<OutboundIter<'a, E>, PredecessorIter<'a, V, E>>;

    fn edges(self, a: Id) -> Self::Edges {
        self.outbound_edges(a).chain(self.predecessors(a))
    }
}

pub enum EdgeIter<'a, V, E> {
    Outgoing(OutboundIter<'a, E>),
    Incoming(PredecessorIter<'a, V, E>),
}

impl<'a, V, E> EdgeIter<'a, V, E> {
    fn from(g: &'a PGraph<V, E>, id: Id, d: Direction) -> Self {
        match d {
            Direction::Outgoing => EdgeIter::Outgoing(g.outbound_edges(id)),
            Direction::Incoming => EdgeIter::Incoming(g.predecessors(id)),
        }
    }
}

impl<'a, V, E> Iterator for EdgeIter<'a, V, E> {
    type Item = (Id, Id, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EdgeIter::Outgoing(iter) => iter.next(),
            EdgeIter::Incoming(iter) => iter.next(),
        }
    }
}

impl<'a, V, E> IntoEdgesDirected for &'a PGraph<V, E> {
    type EdgesDirected = EdgeIter<'a, V, E>;

    fn edges_directed(self, a: Id, dir: Direction) -> Self::EdgesDirected {
        EdgeIter::from(self, a, dir)
    }
}

impl<V, E> NodeCount for PGraph<V, E> {
    fn node_count(&self) -> usize {
        self.empties.len()
    }
}

impl<V, E> NodeIndexable for PGraph<V, E> {
    fn node_bound(&self) -> usize {
        self.node_count()
    }

    fn to_index(&self, a: Id) -> usize {
        let index = a.index();
        let empties_before = self.empties.clone().split(&index).0.len();

        index + empties_before
    }

    fn from_index(&self, i: usize) -> Id {
        let mut i = i;
        for empty in self.empties.iter().cloned() {
            if empty <= i {
                i += 1;
            } else {
                break;
            }
        }
        self.guts[i].as_ref().unwrap().id()
    }
}

impl<V, E> NodeCompactIndexable for PGraph<V, E> {}
