use super::{Id, PGraph, Vertex};
use std::ops::IndexMut;
use std::sync::Arc;

pub struct Edge<'a, V, E> {
    source: &'a mut Vertex<V, E>,
    sink: Id,
}

impl<'a, V: Clone, E> Edge<'a, V, E> {
    pub(crate) fn from(graph: &'a mut PGraph<V, E>, source: Id, sink: Id) -> Self {
        if !graph.has_vertex(sink) {
            panic!(
                "No vertex found for Id {:?}. Cannot construct edge to nonexistent vertex.",
                sink
            )
        } else {
            Self {
                source: graph.index_mut(source),
                sink,
            }
        }
    }
}

impl<'a, V, E> Edge<'a, V, E> {
    pub fn source(&self) -> &Id {
        &self.source.id()
    }

    pub fn sink(&self) -> &Id {
        &self.sink
    }
}

impl<'a, V, E: Clone> Edge<'a, V, E> {
    pub fn or_insert(self, weight: E) -> &'a mut E {
        self.or_insert_with(|| weight)
    }

    pub fn or_insert_with<F: FnOnce() -> E>(self, weight: F) -> &'a mut E {
        let sink = self.sink;
        let edge = self.source.make_edge_mut(sink);
        let (_, weight_arc) = edge.get_or_insert_with(|| (sink, Arc::new(weight())));
        Arc::get_mut(weight_arc).unwrap()
    }

    pub fn and_modify<F: FnOnce(&mut E) -> ()>(self, mutate: F) -> Self {
        self.source.weight_mut(self.sink).map(mutate);
        self
    }
}
impl<'a, V, E: Clone + Default> Edge<'a, V, E> {
    pub fn or_default(self) -> &'a mut E {
        self.or_insert_with(E::default)
    }
}
