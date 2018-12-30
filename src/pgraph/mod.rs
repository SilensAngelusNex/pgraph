use crate::id::{Id, IdGen};
use im::Vector;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::iter::{FilterMap, IntoIterator};
use std::ops::{Index, IndexMut};

mod edge;
mod vertex;

pub use self::edge::Edge;
pub use self::vertex::Vertex;

type GraphInternal<V, E> = Vector<Option<Vertex<V, E>>>;

/// Represents a persistent graph with data on each vertex (of type V) and directed, weighted edges.
/// (Edge weights are of type E.) Uses [Id](struct.Id.html)s as references to vertices.
///
/// All of the `_mut` methods will mutate the PGraph in-place, while the corresponding methods without will clone the existing PGraph and return a modified version.
/// All of the `try_` methods will not panic if their non-`try` counterparts would, and do less redundant cloning.
/// All the graph data is held using structual sharing, so the cloning will be minimally expensive, with respect to both time and memory.
pub struct PGraph<V, E> {
    guts: GraphInternal<V, E>,
    idgen: IdGen,
}

// `derive(Clone)` only implements for <V: Clone, E: Clone> because of rust#26925
impl<V, E> Clone for PGraph<V, E> {
    fn clone(&self) -> Self {
        Self {
            guts: self.guts.clone(),
            idgen: self.idgen.clone(),
        }
    }
}

impl<V, E> Default for PGraph<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Debug, E: Debug> Debug for PGraph<V, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "PGraph ({:?}) {{", self.idgen)?;
        let mut any_vertices = false;

        for v_opt in self.guts.iter() {
            match v_opt {
                Some(v) => write!(f, "\n\t{:?}", v)?,
                None => write!(f, "\n\tBlank")?,
            }
            any_vertices = true;
        }

        writeln!(f, "{}}}", if any_vertices { "\n" } else { "" })?;
        Result::Ok(())
    }
}

// helpers
impl<V, E> PGraph<V, E> {
    /// Gets the current generation of the PGraph's IdGen
    #[cfg(test)]
    #[must_use]
    pub fn generation(&self) -> usize {
        self.idgen.generation()
    }

    /// Counts the number of empty (`None`) slots in the underlying vertex Vector
    #[cfg(test)]
    #[must_use]
    pub fn count_empties(&self) -> usize {
        self.guts
            .iter()
            .filter_map(|v_opt| if v_opt.is_none() { Some(()) } else { None })
            .count()
    }

    /// Finds an empty (`None`) slot in the underlying vector.
    /// Current implementation gets the slot with the first index
    #[must_use]
    fn find_empty(&self) -> Option<usize> {
        for (i, v) in self.guts.iter().enumerate() {
            if v.is_none() {
                return Some(i);
            }
        }
        None
    }
}

impl<V, E> PGraph<V, E> {
    /// Creates a new, empty PGraph
    #[must_use]
    pub fn new() -> Self {
        Self {
            guts: GraphInternal::new(),
            idgen: IdGen::new(),
        }
    }

    /// Checks if the given Id points to a valid [Vertex](struct.Vertex.html). Equivalent to `self.get(id).is_some()`.
    #[must_use]
    pub fn has_vertex(&self, id: Id) -> bool {
        self.vertex(id).is_some()
    }

    /// Gets the [Vertex](struct.Vertex.html) corresponding to a given [Id](struct.Id.html). Will return `None` if one cannot be found.
    ///
    /// Some reasons this could occur are:
    /// -   This `Id` is from a `PGraph` that isn't an ancestor of the current `PGraph`
    /// -   The `Vertex` corresponding to this `Id` has been removed from the `PGraph`
    #[must_use]
    pub fn vertex(&self, id: Id) -> Option<&Vertex<V, E>> {
        match self.guts.get(id.index()) {
            Some(Some(vertex)) if vertex.same_id(id) => Some(&*vertex),
            _ => None,
        }
    }

    /// Gets the data from the [Vertex](struct.Vertex.html) corresponding to a given [Id](struct.Id.html). Will return `None`
    /// if such a [Vertex](struct.Vertex.html) cannot be found. Equivalent to `self.get(id).data()`
    #[must_use]
    pub fn vertex_data(&self, id: Id) -> Option<&V> {
        self.vertex(id).map(|v| v.data())
    }

    /// Returns true iff there exist vertices corresponding to both `source` and `sink` and `source` has an outgoing edge to `sink`.
    #[must_use]
    pub fn has_edge(&self, source: Id, sink: Id) -> bool {
        self.vertex(source).map_or(false, |v| v.is_connected(sink))
    }

    /// If there exists an outgoing edge from `source` to `sink`, returns a reference to that edge's weight. Otherwise, returns `None`.
    #[must_use]
    pub fn weight(&self, source: Id, sink: Id) -> Option<&E> {
        self.vertex(source).and_then(|v| v.weight(sink))
    }

    /// Modifies the PGraph to contain a new vertex containing `data`. (The vertex won't be connected to anything.)
    ///
    /// Returns the new PGraph and the new vertex's Id.
    #[must_use]
    pub fn add(&self, data: V) -> (Self, Id) {
        let mut result = self.clone();
        let id = result.add_mut(data);
        (result, id)
    }

    /// Modifies the PGraph in-place to contain a new vertex containing `data`. (The vertex won't be connected to anything.)  
    /// Returns the new vertex's Id.
    pub fn add_mut(&mut self, data: V) -> Id {
        match self.find_empty() {
            Some(index) => {
                let id = self.idgen.create_id(index);
                self.guts.set(index, Some(Vertex::from(id, data)));
                id
            }
            None => {
                let id = self.idgen.create_id(self.guts.len());
                self.guts.push_back(Some(Vertex::from(id, data)));
                id
            }
        }
    }

    /// Adds multiple vertices to the PGraph. Each contains one of the elements contained in `data_iter`.  
    /// Returns the new PGraph and a Vec of the added [Id](struct.Id.html)s. The order of [Id](struct.Id.html)s in the Vec correspond the position in the `data_iter` from which that vertex's data came.
    ///
    /// Prefer over iterating and calling `add` yourself because this makes less calls to `clone`.
    #[must_use]
    pub fn add_all<T: Into<V>, I: IntoIterator<Item = T>>(&self, data_iter: I) -> (Self, Vec<Id>) {
        let mut result = self.clone();
        let ids = result.add_all_mut(data_iter);
        (result, ids)
    }

    /// Adds multiple vertices to the PGraph in-place. Each contains one of the elements contained in `data_iter`.  
    /// Returns a Vec of the added [Id](struct.Id.html)s. The order of [Id](struct.Id.html)s in the Vec correspond the position in the `data_iter` from which that vertex's data came.
    ///
    /// TODO: Use batch insert when RPDS supports it, so that this is better than iterating and calling `add_mut`
    pub fn add_all_mut<T: Into<V>, I: IntoIterator<Item = T>>(&mut self, vertices: I) -> Vec<Id> {
        vertices
            .into_iter()
            .map(|v| self.add_mut(v.into()))
            .collect()
    }

    /// Returns an iterator over all the valid vertex [Id](struct.Id.html)s in the PGraph
    #[must_use]
    pub fn ids(&self) -> impl Iterator<Item = &Id> {
        self.guts.iter().filter_map(|v_opt| match v_opt {
            Some(v) => Some(v.id()),
            None => None,
        })
    }

    /// Returns an iterator over all the valid vertex data in the PGraph
    #[must_use]
    pub fn iter_data(&self) -> impl Iterator<Item = &V> {
        self.guts.iter().filter_map(|v_opt| match v_opt {
            Some(v) => Some(v.data()),
            None => None,
        })
    }

    /// Returns an iterator over all wieghts of edges existing in the PGraph
    #[must_use]
    pub fn iter_weights(&self) -> impl Iterator<Item = &E> {
        self.guts
            .iter()
            .filter_map(|v_opt| match v_opt {
                Some(v) => Some(v.into_iter()),
                None => None,
            })
            .flatten()
            .map(|(_, weight)| weight)
    }

    /// Returns an iterator over all vertices in the PGraph with an edge that _ends_ at `sink`.
    #[must_use]
    pub fn predecessors(&self, sink: Id) -> impl Iterator<Item = &Id> {
        self.guts.iter().filter_map(move |v_opt| match v_opt {
            Some(v) if v.is_connected(sink) => Some(v.id()),
            _ => None,
        })
    }

    /// Returns an iterator over all wieghts of edges existing in the PGraph that _start_ at `source`.
    #[must_use]
    pub fn outbound_edges(&self, source: Id) -> impl Iterator<Item = (&Id, &E)> {
        self.vertex(source)
            .map(|v| v.into_iter())
            .into_iter()
            .flatten()
    }
}

impl<V: Clone, E> PGraph<V, E> {
    /// Gets a mutable reference data from the [Vertex](struct.Vertex.html) corresponding to a given [Id](struct.Id.html). Will return `None`
    /// if such a [Vertex](struct.Vertex.html) cannot be found. Equivalent to `self.get_mut(id).data_mut()`
    #[must_use]
    pub fn vertex_data_mut(&mut self, id: Id) -> Option<&mut V> {
        self.vertex_mut(id).map(|v| v.data_mut())
    }

    /// Creates an edge from `source` to `sink`. If there already exists an edge, it will be overwritten. (Vertices can have edges to themselves.)
    ///
    /// Returns the new, modified version of the PGraph.  
    /// Panics if `source` and/or `sink` is not in the PGraph
    #[must_use]
    pub fn connect(&self, source: Id, sink: Id, weight: E) -> Self {
        let mut result = self.clone();
        result.connect_mut(source, sink, weight);
        result
    }

    /// Tries to create an edge from `source` to `sink`. If there already exists an edge, it will be overwritten. (Vertices can have edges to themselves.)
    ///
    /// Returns the new, modified version of the PGraph, or `None` if `source` and/or `sink` is not in the PGraph.
    #[must_use]
    pub fn try_connect(&self, source: Id, sink: Id, weight: E) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if result.has_vertex(source) && result.has_vertex(sink) {
            result.to_mut().connect_mut(source, sink, weight);
            Some(result.into_owned())
        } else {
            None
        }
    }

    /// Creates an edge from `source` to `sink`, in-place. If there already exists an edge, it will be overwritten. (Vertices can have edges to themselves.)
    ///
    /// Panics if `source` and/or `sink` is not in the PGraph
    pub fn connect_mut(&mut self, source: Id, sink: Id, weight: E) {
        if self.has_vertex(sink) {
            self[source].connect_to(sink, weight);
        } else {
            panic!(
                "The sink vertex with Id {:?} was not found in the graph.",
                sink
            )
        }
    }

    /// Tries to create an edge from `source` to `sink`. If there already exists an edge, it will be overwritten. (Vertices can have edges to themselves.)
    ///
    /// Returns `false` iff the edge couldn't be created (i.e. `source` and/or `sink` is not in the PGraph)
    pub fn try_connect_mut(&mut self, source: Id, sink: Id, weight: E) -> bool {
        if self.has_vertex(sink) {
            if let Some(v) = self.vertex_mut(source) {
                v.connect_to(sink, weight);
                return true;
            }
        };
        false
    }

    /// Gets a mutable reference to the [Vertex](struct.Vertex.html) corresponding to a given [Id](struct.Id.html). Will return `None` if one cannot be found.
    ///
    /// Some reasons this could occur are:
    /// -   This `Id` is from a `PGraph` that isn't an ancestor of the current `PGraph`
    /// -   The `Vertex` corresponding to this `Id` has been removed from the `PGraph`
    #[must_use]
    pub fn vertex_mut(&mut self, id: Id) -> Option<&mut Vertex<V, E>> {
        match self.guts.get_mut(id.index()) {
            Some(Some(vertex)) if vertex.same_id(id) => Some(vertex),
            _ => None,
        }
    }
}

impl<V: Clone, E: Clone> PGraph<V, E> {
    /// Recreates a graph from scratch, so that it and the old graph have no shared structure.
    /// This means that the [Id](struct.Id.html)s from the old graph will not work on the new one.
    #[must_use]
    pub fn recreate(&self) -> Self {
        let mut result = Self::new();
        let mut ids = HashMap::new();
        for v in self {
            ids.insert(v.id(), result.add_mut(v.data().clone()));
        }

        for source in self {
            for (sink, weight) in source {
                result.connect_mut(ids[source.id()], ids[sink], weight.clone())
            }
        }
        result
    }

    /// If there exists an outgoing edge from `source` to `sink`, returns a mutable reference to that edge's weight. Otherwise, returns `None`.
    #[must_use]
    pub fn weight_mut(&mut self, source: Id, sink: Id) -> Option<&mut E> {
        self.vertex_mut(source).and_then(|v| v.weight_mut(sink))
    }

    /// Creates an [Edge](struct.Edge.html), which functions like HashMap's Entry, that can be used to connect `source` and `sink`
    /// if there is no existing edge, or modify the edge if there is one.
    #[must_use]
    pub fn edge(&mut self, source: Id, sink: Id) -> Edge<V, E> {
        Edge::from(self, source, sink)
    }

    /// Removes a vertex and all edges from and to it from the PGraph.
    ///
    /// Returns the modified PGraph, which may be identical to the PGraph passed in if the vertix didn't exist.
    #[must_use]
    pub fn remove(&self, id: Id) -> Self {
        let mut result = self.clone();
        result.remove_mut(id);
        result
    }

    /// Tries to remove a vertex and all edges from and to it from the PGraph.
    ///
    /// Returns the modified PGraph if the vertex existed to be removed, `None` otherwise.
    #[must_use]
    pub fn try_remove(&self, id: Id) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if remove(&mut result, id) {
            Some(result.into_owned())
        } else {
            None
        }
    }

    /// Removes multiple vertices and all edges from and to them from the PGraph.
    ///
    /// Returns the modified PGraph, which may be identical to the PGraph passed in if none of the vertices existed.
    #[must_use]
    pub fn remove_all<T: Into<Id>, I: IntoIterator<Item = T>>(&self, iterable: I) -> Self {
        let mut result = self.clone();
        result.remove_all_mut(iterable);
        result
    }

    /// Remove multiple vertices and all edges from and to them from the PGraph.
    ///
    /// Returns the modified PGraph if one or more of the vertices existed to be removed, otherwise `None`.
    #[must_use]
    pub fn try_remove_all<T: Into<Id>, I: IntoIterator<Item = T>>(
        &self,
        iterable: I,
    ) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if remove_all(&mut result, iterable) {
            Some(result.into_owned())
        } else {
            None
        }
    }

    /// Removes the edge from `source` to `sink`, if one exists. Panics if `source` doesn't exist.
    ///
    /// Returns the modified PGraph
    #[must_use]
    pub fn disconnect(&self, source: Id, sink: Id) -> Self {
        let mut result = self.clone();
        result.disconnect_mut(source, sink);
        result
    }

    /// Tries to remove the edge from `source` to `sink`, if one exists.
    ///
    /// If both `source` and `sink` exist and there existed an edge from `source` to `sink` to be removed, returns the modified PGraph.
    /// Otherwise, returns `None`.
    #[must_use]
    pub fn try_disconnect(&self, source: Id, sink: Id) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if result.has_vertex(source) && result.has_vertex(sink) {
            result.to_mut().disconnect_mut(source, sink);
            Some(result.into_owned())
        } else {
            None
        }
    }

    /// Removes a vertex and all edges from and to it from the PGraph.
    ///
    /// Returns `true` if the vertex existed to be removed, `false` otherwise.
    pub fn remove_mut(&mut self, id: Id) -> bool {
        if self.has_vertex(id) {
            self.remove_mut_no_inc(id);
            self.idgen.next_gen();
            true
        } else {
            false
        }
    }

    /// Removes multiple vertices and all edges from and to them from the PGraph.
    ///
    /// Returns `true` if one or more vertices existed to be removed, `false` otherwise.
    pub fn remove_all_mut<T: Into<Id>, I: IntoIterator<Item = T>>(&mut self, iter: I) -> bool {
        let changed = self.remove_all_mut_no_inc(iter);
        if changed {
            self.idgen.next_gen();
        };
        changed
    }

    /// Removes multiple vertices without incrementing the PGraph's generation.
    #[must_use]
    fn remove_all_mut_no_inc<T: Into<Id>, I: IntoIterator<Item = T>>(
        &mut self,
        iterable: I,
    ) -> bool {
        iterable.into_iter().fold(false, |changed, into_id| {
            self.try_remove_mut_no_inc(into_id.into()) || changed
        })
    }

    /// Checks if a vertex exists, then removes it without incrementing the PGraph's generation.
    #[must_use]
    fn try_remove_mut_no_inc(&mut self, id: Id) -> bool {
        if self.has_vertex(id) {
            self.remove_mut_no_inc(id);
            true
        } else {
            false
        }
    }

    /// Removes a vertex without incrementing the PGraph's generation.
    fn remove_mut_no_inc(&mut self, id: Id) {
        self.guts.set(id.index(), None);
        self.disconnect_all_inc_mut(id);
    }

    /// Removes the edge from `source` to `sink`, if one exists. Panics if `source` doesn't exist.
    ///
    /// Returns `true` if there was previously an edge from `source` to `sink`
    pub fn disconnect_mut(&mut self, source: Id, sink: Id) -> bool {
        self[source].disconnect(sink)
    }

    /// Tries to remove the edge from `source` to `sink`, if one exists.
    ///
    /// Returns `true` if there was previously an edge from `source` to `sink`
    pub fn try_disconnect_mut(&mut self, source: Id, sink: Id) -> bool {
        self.vertex_mut(source)
            .map_or(false, |v| v.disconnect(sink))
    }

    /// Disconnects all the edges that end at `sink`.
    fn disconnect_all_inc_mut(&mut self, sink: Id) -> bool {
        let inc: Vec<Id> = self.predecessors(sink).cloned().collect();
        inc.into_iter().fold(false, |init, source| {
            self.disconnect_mut(source, sink) || init
        })
    }
}

impl<V, E> Index<Id> for PGraph<V, E> {
    type Output = Vertex<V, E>;

    fn index(&self, id: Id) -> &Vertex<V, E> {
        match self.guts.get(id.index()) {
            Some(Some(vertex)) if vertex.same_id(id) => vertex,
            Some(Some(_)) => panic!("The Id {:?} is of an invalid generation. It does not correspond to any vertices in this graph.", id),
            Some(None) => panic!("No vertex found for Id {:?}. It has likely been removed from the graph.", id),
            None => panic!("No vertex found for Id {:?}. The Id either comes from a chlid or another graph family.", id),
        }
    }
}

impl<V: Clone, E> IndexMut<Id> for PGraph<V, E> {
    fn index_mut(&mut self, id: Id) -> &mut Vertex<V, E> {
        match self.guts.get_mut(id.index()) {
            Some(Some(vertex)) if vertex.same_id(id) => vertex,
            Some(Some(_)) => panic!("The Id {:?} is of an invalid generation. It does not correspond to any vertices in this graph", id),
            Some(None) => panic!("No vertex found for Id {:?}. It has likely been removed from the graph.", id),
            None => panic!("No vertex found for Id {:?}. The Id either comes from a chlid or another graph family.", id),
        }
    }
}

impl<V, E> Index<(Id,)> for PGraph<V, E> {
    type Output = V;

    fn index(&self, id: (Id,)) -> &V {
        self[id.0].data()
    }
}

impl<V: Clone, E> IndexMut<(Id,)> for PGraph<V, E> {
    fn index_mut(&mut self, id: (Id,)) -> &mut V {
        self[id.0].data_mut()
    }
}

impl<V, E> Index<(Id, Id)> for PGraph<V, E> {
    type Output = E;

    fn index(&self, ids: (Id, Id)) -> &E {
        let (source, sink) = ids;
        self[source].index(sink)
    }
}

impl<V: Clone, E: Clone> IndexMut<(Id, Id)> for PGraph<V, E> {
    fn index_mut(&mut self, ids: (Id, Id)) -> &mut E {
        let (source, sink) = ids;
        self[source].index_mut(sink)
    }
}

/// Tries to remove a vertex if it exists in the PGraph, only cloning if that PGraph will actually be modified.
fn remove<'a, V: Clone, E: Clone>(cow: &mut Cow<'a, PGraph<V, E>>, id: Id) -> bool {
    if cow.has_vertex(id) {
        cow.to_mut().remove_mut_no_inc(id);
        true
    } else {
        false
    }
}

/// Tries to remove multiple vertices if it exists in the PGraph, only cloning if that PGraph will actually be modified.
fn remove_all<'a, V: Clone, E: Clone, T: Into<Id>, I: IntoIterator<Item = T>>(
    cow: &mut Cow<'a, PGraph<V, E>>,
    iterable: I,
) -> bool {
    iterable.into_iter().fold(false, |changed, into_id| {
        remove(cow, into_id.into()) || changed
    })
}

type GutsIter<'a, V, E> = <&'a GraphInternal<V, E> as IntoIterator>::IntoIter;
type VertexDeref<'a, V, E> = fn(&'a Option<Vertex<V, E>>) -> Option<&'a Vertex<V, E>>;

impl<'a, V, E> IntoIterator for &'a PGraph<V, E> {
    type Item = &'a Vertex<V, E>;
    type IntoIter = FilterMap<GutsIter<'a, V, E>, VertexDeref<'a, V, E>>;

    fn into_iter(self) -> Self::IntoIter {
        self.guts.iter().filter_map(|v_opt| v_opt.as_ref())
    }
}
