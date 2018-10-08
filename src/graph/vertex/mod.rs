use self::adj::AdjList;
use id::Id;
use std::fmt::{Debug, Error, Formatter};
use std::iter::IntoIterator;
use std::ops::Index;

pub mod adj;

///
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
    pub fn get_data(&self) -> &V {
        &self.data
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub fn get_cost(&self, sink: &Id) -> Option<&E> {
        self.adj.get_edge(sink)
    }

    pub fn is_connected(&self, sink: &Id) -> bool {
        self.adj.has_edge(sink)
    }

    pub fn len_neighbors(&self) -> usize {
        self.adj.len()
    }

    pub fn get_data_mut(&mut self) -> &mut V {
        &mut self.data
    }

    pub(crate) fn from(id: Id, data: V) -> Self {
        Vertex {
            id,
            data,
            adj: AdjList::new(),
        }
    }

    pub(crate) fn same_id(&self, id: &Id) -> bool {
        id == &self.id
    }

    pub(crate) fn connect_to(&mut self, sink: &Id, weight: E) {
        self.adj.add_edge(sink, weight)
    }
}

impl<V, E: Clone> Vertex<V, E> {
    pub fn get_cost_mut(&mut self, sink: &Id) -> Option<&mut E> {
        self.adj.get_edge_mut(sink)
    }

    pub fn disconnect(&mut self, sink: &Id) -> bool {
        self.adj.disconnect_edge(sink)
    }
}

impl<'a, V, E> Index<&'a Id> for Vertex<V, E> {
    type Output = E;

    fn index(&self, id: &'a Id) -> &E {
        self.adj.index(id)
    }
}

impl<'a, V, E> IntoIterator for &'a Vertex<V, E> {
    type Item = adj::IterItem<'a, E>;
    type IntoIter = adj::Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.adj.into_iter()
    }
}
