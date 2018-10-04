use id::Id;
use rpds::Vector;
use std::fmt::{Debug, Error, Formatter};
use std::iter::{FilterMap, IntoIterator};
use std::ops::{Index, IndexMut};

pub type Edge<E> = (Id, E);

pub(crate) struct AdjList<E> {
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
        let mut iter = self.iter();
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
    pub(crate) fn new() -> Self {
        AdjList {
            edges: Vector::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Edge<E>> {
        self.edges.iter().filter_map(|e| e.as_ref())
    }

    pub(crate) fn has_edge(&self, sink: &Id) -> bool {
        self.get_edge(sink).is_some()
    }

    pub(crate) fn get_edge(&self, sink: &Id) -> Option<&E> {
        let e = self.edges.get(sink.into());
        if let Some(Some((id, weight))) = e {
            if sink == id {
                Some(weight)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn add_edge(&mut self, sink: &Id, weight: E) {
        let mut new_edges = self.edges.clone();
        while new_edges.len() <= sink.into() {
            new_edges.push_back_mut(None);
        }
        new_edges.set_mut(sink.into(), Some((*sink, weight)));
        self.edges = new_edges;
    }
}

impl<E: Clone> AdjList<E> {
    pub(crate) fn get_edge_mut(&mut self, sink: &Id) -> Option<&mut E> {
        let e = self.edges.get_mut(sink.into());
        if let Some(Some((id, weight))) = e {
            if sink == id {
                Some(weight)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn disconnect_edge(&mut self, sink: &Id) -> bool {
        let mut result = false;
        let e = self.edges.get_mut(sink.into());
        if let Some(edge) = e {
            let take = if let Some((id, _)) = edge {
                sink == id
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
        let mut iter1 = self.iter();
        let mut iter2 = other.iter();

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

impl<E> Index<Id> for AdjList<E> {
    type Output = E;

    fn index(&self, id: Id) -> &E {
        self.get_edge(&id).unwrap()
    }
}

impl<E: Clone> IndexMut<Id> for AdjList<E> {
    fn index_mut(&mut self, id: Id) -> &mut E {
        self.get_edge_mut(&id).unwrap()
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
