use id::Id;
use rpds::Vector;
use std::fmt::{Debug, Error, Formatter};
use std::iter::{FilterMap, IntoIterator};
use std::ops::{Index, IndexMut};

/// Contains the `Id` of the sink vertex and the weight
pub type Edge<E> = (Id, E);

/// Struct to manage a vertex's adjacencies without having to care about the vertex itself.
pub(super) struct AdjList<E> {
    edges: Vector<Option<Edge<E>>>,
}

// Derive only implements for <E: Clone> because of rust#26925
impl<E> Clone for AdjList<E> {
    fn clone(&self) -> Self {
        AdjList {
            edges: self.edges.clone(),
        }
    }
}

impl<E: Debug> Debug for AdjList<E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut iter = self.into_iter();
        match iter.next() {
            Some((id, weight)) => write!(f, "{:?}: {:?}", id, weight)?,
            None => write!(f, "None")?,
        };

        for (id, weight) in iter {
            write!(f, ", {:?}: {:?}", id, weight)?;
        }

        Result::Ok(())
    }
}

impl<E> AdjList<E> {
    /// Creates a new, empty `AdjList`
    pub(super) fn new() -> Self {
        AdjList {
            edges: Vector::new(),
        }
    }

    /// Counts the number of neighbors.
    ///
    /// Takes O(N) time where N is the maximum number of vertices that _have ever been_ in the graph.  
    /// TODO: Store this if it comes up a lot?
    pub(super) fn len(&self) -> usize {
        self.into_iter().count()
    }

    /// Returns true iff there exists an `Edge` that goes to `sink`.
    ///
    /// Runs in O(1)
    pub(super) fn has_edge(&self, sink: Id) -> bool {
        self.get_edge(sink).is_some()
    }

    /// Returns the `Edge` that goes to `sink`, or `None` if such an edge doesn't exist.
    ///
    /// Runs in O(1)
    pub(super) fn get_edge(&self, sink: Id) -> Option<&E> {
        let e = self.edges.get(sink.index());
        if let Some(Some((id, weight))) = e {
            if sink == *id {
                Some(weight)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Create an edge to `sink` with the given `weight`.
    ///
    /// Worst case runs in O(N), where N is the maximum number of vertices *currently* in the graph. Amortized O(1).
    pub(super) fn add_edge(&mut self, sink: Id, weight: E) {
        let mut new_edges = self.edges.clone();
        while new_edges.len() <= sink.index() {
            new_edges.push_back_mut(None);
        }
        new_edges.set_mut(sink.index(), Some((sink, weight)));
        self.edges = new_edges;
    }
}

impl<E: Clone> AdjList<E> {
    /// Gets a mutable reference to the weight of the edge that ends at `sink`, or `None` if no such edge exists.
    ///
    /// Runs in O(1)
    pub(super) fn get_edge_mut(&mut self, sink: Id) -> Option<&mut E> {
        let e = self.edges.get_mut(sink.index());
        if let Some(Some((id, weight))) = e {
            if sink == *id {
                Some(weight)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Deletes the edge that ends at `sink`. Returns false iff that edge didn't exist to begin with.
    ///
    /// Runs in O(1)
    pub(super) fn disconnect_edge(&mut self, sink: Id) -> bool {
        let mut result = false;
        let e = self.edges.get_mut(sink.index());
        if let Some(edge) = e {
            let take = if let Some((id, _)) = edge {
                sink == *id
            } else {
                false
            };

            if take {
                edge.take();
                result = true;
            }
        };
        result
    }
}

impl<E: PartialEq<F>, F> PartialEq<AdjList<F>> for AdjList<E> {
    fn eq(&self, other: &AdjList<F>) -> bool {
        let mut iter1 = self.into_iter();
        let mut iter2 = other.into_iter();

        let item1 = iter1.next();
        let item2 = iter2.next();

        loop {
            match item1 {
                Some((id1, e1)) => match item2 {
                    Some((id2, e2)) if id1 == id2 && e1 == e2 => (),
                    _ => break false,
                },
                None => match item2 {
                    Some(_) => break false,
                    None => break true,
                },
            }
        }
    }
}

impl<'a, E> Index<Id> for AdjList<E> {
    type Output = E;

    fn index(&self, id: Id) -> &E {
        self.get_edge(id).unwrap()
    }
}

impl<'a, E: Clone> IndexMut<Id> for AdjList<E> {
    fn index_mut(&mut self, id: Id) -> &mut E {
        self.get_edge_mut(id).unwrap()
    }
}

pub type IterItem<'a, E> = &'a Edge<E>;
pub type Iter<'a, E> = FilterMap<
    <&'a Vector<Option<Edge<E>>> as IntoIterator>::IntoIter,
    fn(&'a Option<Edge<E>>) -> Option<IterItem<'a, E>>,
>;

impl<'a, E> IntoIterator for &'a AdjList<E> {
    type Item = IterItem<'a, E>;
    type IntoIter = Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.iter().filter_map(|e| e.as_ref())
    }
}
