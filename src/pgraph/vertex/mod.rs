use self::adj::AdjList;
use id::Id;
use std::fmt::{Debug, Error, Formatter};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

pub mod adj;

/// Holds the data for a single PGraph vertex. Contains the vertex [Id](struct.Id.html), the vertex's data, and the vertex's neighbors.
pub struct Vertex<V, E> {
    id: Id,
    data: V,
    adj: AdjList<E>,
}

impl<V: Debug, E: Debug> Debug for Vertex<V, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} ({:?}) => ({:?})", self.data, self.id, self.adj)
    }
}

impl<V: Clone, E> Clone for Vertex<V, E> {
    fn clone(&self) -> Self {
        Vertex {
            id: self.id,
            data: self.data.clone(),
            adj: self.adj.clone(),
        }
    }
}

impl<V, E> Vertex<V, E> {
    /// Returns a reference to the data on this vertex
    #[must_use]
    pub fn data(&self) -> &V {
        &self.data
    }

    /// Returns this vertex's Id
    #[must_use]
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Returns the weight of the edge from this vertex to `sink`, or `None` if such an edge doesn't exist.
    #[must_use]
    pub fn get_cost(&self, sink: Id) -> Option<&E> {
        self.adj.get_edge(sink)
    }

    /// Returns `true` iff there exists an edge from this vertex to `sink`
    #[must_use]
    pub fn is_connected(&self, sink: Id) -> bool {
        self.adj.has_edge(sink)
    }

    /// Returns the number of outgoing edges this vertex has.
    #[must_use]
    pub fn len_neighbors(&self) -> usize {
        self.adj.len()
    }

    /// Returns a mutable reference to the data on this vertex
    #[must_use]
    pub fn data_mut(&mut self) -> &mut V {
        &mut self.data
    }

    /// Creates a vertex from an Id and vertex data. The vertex starts with no neighbors.
    #[must_use]
    pub(super) fn from(id: Id, data: V) -> Self {
        Vertex {
            id,
            data,
            adj: AdjList::new(),
        }
    }

    /// Checks if the given Id has the same generation and index as this vertex's Id
    #[must_use]
    pub(super) fn same_id(&self, id: Id) -> bool {
        id == self.id
    }

    /// Creates an edge to `sink` with the given weight.
    ///
    /// This is `pub(super)` instead of `pub` because the vertex has no way to check whether `sink` actually exists in the PGraph.
    pub(super) fn connect_to(&mut self, sink: Id, weight: E) {
        self.adj.add_edge(sink, weight)
    }
}

impl<V, E: Clone> Vertex<V, E> {
    /// Returns a mutable reference to the weight of the edge from this vertex to sink, or `None` if one doesn't exist.
    #[must_use]
    pub fn get_cost_mut(&mut self, sink: Id) -> Option<&mut E> {
        self.adj.get_edge_mut(sink)
    }

    /// Removes the edge from this vertex to `sink`.
    ///
    /// Returns `true` iff the edge existed to be removed.
    pub fn disconnect(&mut self, sink: Id) -> bool {
        self.adj.disconnect_edge(sink)
    }
}

impl<'a, V, E> Index<Id> for Vertex<V, E> {
    type Output = E;

    fn index(&self, id: Id) -> &E {
        self.adj.index(id)
    }
}

impl<'a, V, E: Clone> IndexMut<Id> for Vertex<V, E> {
    fn index_mut(&mut self, id: Id) -> &mut E {
        self.adj.index_mut(id)
    }
}

impl<'a, V, E> IntoIterator for &'a Vertex<V, E> {
    type Item = adj::IterItem<'a, E>;
    type IntoIter = adj::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.adj.into_iter()
    }
}
