use super::{Id, PGraph, Vertex};
use std::borrow::Borrow;
use std::ops::IndexMut;
use std::sync::Arc;

pub struct Edge<'a, V, E> {
    source: &'a mut Vertex<V, E>,
    sink: Id,
}

impl<'a, V: Clone, E> Edge<'a, V, E> {
    /// Creates an Edge for the edge from `source` to `sink`. (This method can't be on [`Vertex`](structs.Vertex.html)
    /// because the vertex has no way of checking whether the `sink` vertex actually exists in the graph.)
    pub(crate) fn from<T: Borrow<Id>>(graph: &'a mut PGraph<V, E>, source: T, sink: T) -> Self {
        let sink = sink.borrow();

        if !graph.has_vertex(sink) {
            panic!(
                "No vertex found for Id {:?}. Cannot construct edge to nonexistent vertex.",
                sink
            )
        } else {
            Self {
                source: graph.index_mut(source),
                sink: *sink,
            }
        }
    }
}

impl<'a, V, E> Edge<'a, V, E> {
    /// Returns the [`Id`](structs.Id.html) of the source vertex.
    pub fn source(&self) -> Id {
        self.source.id()
    }

    /// Returns the [`Id`](structs.Id.html) of the sink vertex.
    pub fn sink(&self) -> Id {
        self.sink
    }
}

impl<'a, V, E: Clone> Edge<'a, V, E> {
    /// Ensures this edge has a weight by inserting `default` if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// Since the value of `default` will be calculated even if it is not needed,
    /// prefer `or_insert_with` when work has to be done to construct the default.
    /// # Examples
    ///
    /// ```
    /// # use pgraph::PGraph;
    /// # fn main() {
    /// let mut g = PGraph::<&str, usize>::new();
    ///
    /// let v1 = g.add_mut("A");
    /// let v2 = g.add_mut("B");
    ///
    /// g.connect_mut(v1, v2, 7);
    /// let fore = g.edge(v1, v2)
    ///     .or_insert(0);
    /// *fore += 4;
    /// assert_eq!(11, g[(v1, v2)]);
    ///
    /// let back = g.edge(v2, v1)
    ///     .or_insert(0);
    /// *back += 4;
    /// assert_eq!(4, g[(v2, v1)]);
    /// # }
    /// ```
    pub fn or_insert(self, default: E) -> &'a mut E {
        let sink = self.sink;
        let edge = self.source.make_edge_mut(sink);
        let (_, weight_arc) = edge.get_or_insert((sink, Arc::new(default)));
        Arc::get_mut(weight_arc).unwrap()
    }

    /// Ensures this edge has a weight by inserting the result of the `default` function
    /// if empty, and returns a mutable reference to the value in the entry.
    ///
    /// Since the `default` fuction will only be called if its result is needed, `or_insert_with`
    /// should generally be used when work has to be done to construct the default.
    /// # Examples
    ///
    /// ```
    /// # use pgraph::PGraph;
    /// # fn main() {
    /// let mut g = PGraph::<&str, usize>::new();
    ///
    /// let v1 = g.add_mut("A");
    /// let v2 = g.add_mut("B");
    ///
    /// g.connect_mut(v1, v2, 7);
    /// let fore = g.edge(v1, v2)
    ///     .or_insert_with(|| {
    ///         // Some complex initilization
    ///         0
    ///     });
    /// *fore += 4;
    /// assert_eq!(11, g[(v1, v2)]);
    ///
    /// let back = g.edge(v2, v1)
    ///     .or_insert_with(|| {
    ///         // Some complex initilization
    ///         0
    ///     });
    /// *back += 4;
    /// assert_eq!(4, g[(v2, v1)]);
    /// # }
    /// ```
    pub fn or_insert_with<F: FnOnce() -> E>(self, default: F) -> &'a mut E {
        let sink = self.sink;
        let edge = self.source.make_edge_mut(sink);
        let (_, weight_arc) = edge.get_or_insert_with(|| (sink, Arc::new(default())));
        Arc::get_mut(weight_arc).unwrap()
    }

    /// Provides in-place mutable access to an existing edge weight before any potential edge creation.
    /// # Examples
    ///
    /// ```
    /// # use pgraph::PGraph;
    /// # fn main() {
    /// let mut g = PGraph::<&str, usize>::new();
    ///
    /// let v1 = g.add_mut("A");
    /// let v2 = g.add_mut("B");
    ///
    /// g.edge(v1, v2)
    ///     .and_modify(|weight| *weight += 7)
    ///     .or_default();
    /// assert_eq!(0, g[(v1, v2)]);
    ///
    /// g.edge(v1, v2)
    ///     .and_modify(|weight| *weight += 13)
    ///     .or_default();
    /// assert_eq!(13, g[(v1, v2)]);
    ///
    /// g.edge(v2, v1)
    ///     .and_modify(|weight| *weight = 343);
    /// assert!(!g.has_edge(v2, v1));
    /// # }
    /// ```
    pub fn and_modify<F: FnOnce(&mut E) -> ()>(self, mutate: F) -> Self {
        self.source.weight_mut(self.sink).map(mutate);
        self
    }
}
impl<'a, V, E: Clone + Default> Edge<'a, V, E> {
    /// Ensures this edge has a weight by inserting the default if empty,
    /// and returns a mutable reference to the value in the entry.
    /// # Examples
    ///
    /// ```
    /// # use pgraph::PGraph;
    /// # fn main() {
    /// let mut g = PGraph::<&str, usize>::new();
    ///
    /// let v1 = g.add_mut("A");
    /// let v2 = g.add_mut("B");
    ///
    /// g.edge(v1, v2).or_default();
    /// assert_eq!(0, g[(v1, v2)]);
    /// # }
    /// ```
    pub fn or_default(self) -> &'a mut E {
        self.or_insert_with(E::default)
    }
}
