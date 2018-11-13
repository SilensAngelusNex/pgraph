use self::vertex::adj::Edge;
use self::vertex::Vertex;
use id::{Id, IdGen};
use rpds::Vector;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::iter::{FilterMap, IntoIterator};
use std::ops::{Index, IndexMut};

pub mod vertex;

type GraphInternal<V, E> = Vector<Option<Vertex<V, E>>>;

pub struct Graph<V, E> {
    guts: GraphInternal<V, E>,
    idgen: IdGen,
}

// Derive only implements for <V: Clone, E: Clone> because of rust#26925
impl<V, E> Clone for Graph<V, E> {
    fn clone(&self) -> Self {
        Self {
            guts: self.guts.clone(),
            idgen: self.idgen.clone(),
        }
    }
}

impl<V, E> Default for Graph<V, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Debug, E: Debug> Debug for Graph<V, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Graph (gen {}) {{", self.idgen.get_gen())?;
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
impl<V, E> Graph<V, E> {
    #[cfg(test)]
    pub(crate) fn get_gen(&self) -> usize {
        self.idgen.get_gen()
    }

    #[cfg(test)]
    pub(crate) fn count_empties(&self) -> usize {
        self.guts
            .iter()
            .filter_map(|v_opt| if v_opt.is_none() { Some(()) } else { None })
            .count()
    }

    fn find_empty(&self) -> Option<usize> {
        for (i, v) in self.guts.iter().enumerate() {
            if v.is_none() {
                return Some(i);
            }
        }
        None
    }
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Self {
        Self {
            guts: GraphInternal::new(),
            idgen: IdGen::new(),
        }
    }

    pub fn has_vertex(&self, id: Id) -> bool {
        self.get(id).is_some()
    }

    pub fn get(&self, id: Id) -> Option<&Vertex<V, E>> {
        match self.guts.get(id.get_index()) {
            Some(Some(vertex)) if vertex.same_id(id) => Some(vertex),
            _ => None,
        }
    }

    pub fn get_data(&self, id: Id) -> Option<&V> {
        self.get(id).map(|v| v.get_data())
    }

    pub fn has_edge(&self, source: Id, sink: Id) -> bool {
        self.get(source).map_or(false, |v| v.is_connected(sink))
    }

    pub fn get_edge(&self, source: Id, sink: Id) -> Option<&E> {
        self.get(source).and_then(|v| v.get_cost(sink))
    }

    pub fn add(&self, vertex: V) -> (Self, Id) {
        let mut result = self.clone();
        let id = result.add_mut(vertex);
        (result, id)
    }

    pub fn add_mut(&mut self, vertex: V) -> Id {
        match self.find_empty() {
            Some(index) => {
                let id = self.idgen.create_id(index);
                self.guts.set_mut(index, Some(Vertex::from(id, vertex)));
                id
            }
            None => {
                let id = self.idgen.create_id(self.guts.len());
                self.guts.push_back_mut(Some(Vertex::from(id, vertex)));
                id
            }
        }
    }

    pub fn ids(&self) -> impl Iterator<Item = &Id> {
        self.guts.iter().filter_map(|v_opt| match v_opt {
            Some(v) => Some(v.get_id()),
            None => None,
        })
    }

    pub fn iter_data(&self) -> impl Iterator<Item = &V> {
        self.guts.iter().filter_map(|v_opt| match v_opt {
            Some(v) => Some(v.get_data()),
            None => None,
        })
    }

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

    pub fn vertices_with_edge_to(&self, sink: Id) -> impl Iterator<Item = &Id> {
        self.guts.iter().filter_map(move |v_opt| match v_opt {
            Some(v) if v.is_connected(sink) => Some(v.get_id()),
            _ => None,
        })
    }

    pub fn neighbors(&self, source: Id) -> impl Iterator<Item = &Edge<E>> {
        self.get(source)
            .map(|v| v.into_iter())
            .into_iter()
            .flatten()
    }
}

impl<V: Clone, E> Graph<V, E> {
    pub fn get_data_mut(&mut self, id: Id) -> Option<&mut V> {
        self.get_mut(id).map(|v| v.get_data_mut())
    }

    pub fn connect(&self, source: Id, sink: Id, edge: E) -> Self {
        let mut result = self.clone();
        result.connect_mut(source, sink, edge);
        result
    }

    pub fn try_connect(&self, source: Id, sink: Id, edge: E) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if result.has_vertex(source) && result.has_vertex(sink) {
            result.to_mut().connect_mut(source, sink, edge);
            Some(result.into_owned())
        } else {
            None
        }
    }

    pub fn connect_mut(&mut self, source: Id, sink: Id, edge: E) {
        if self.has_vertex(sink) {
            self[source].connect_to(sink, edge)
        } else {
            panic!(
                "The sink vertex with Id {:?} was not found in the graph.",
                sink
            )
        }
    }

    pub fn try_connect_mut(&mut self, source: Id, sink: Id, edge: E) -> bool {
        if self.has_vertex(sink) {
            if let Some(v) = self.get_mut(source) {
                v.connect_to(sink, edge);
                return true;
            }
        };
        false
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut Vertex<V, E>> {
        match self.guts.get_mut(id.get_index()) {
            Some(Some(vertex)) if vertex.same_id(id) => Some(vertex),
            _ => None,
        }
    }
}

impl<V: Clone, E: Clone> Graph<V, E> {
    pub fn recreate(&self) -> Self {
        let mut result = Self::new();
        let mut ids = HashMap::new();
        for v in self {
            ids.insert(v.get_id(), result.add_mut(v.get_data().clone()));
        }

        for source in self {
            for (sink, weight) in source {
                result.connect_mut(ids[source.get_id()], ids[sink], weight.clone())
            }
        }
        result
    }

    pub fn get_edge_mut(&mut self, source: Id, sink: Id) -> Option<&mut E> {
        self.get_mut(source).and_then(|v| v.get_cost_mut(sink))
    }

    pub fn remove(&self, id: Id) -> Self {
        let mut result = self.clone();
        result.remove_mut(id);
        result
    }

    pub fn try_remove(&self, id: Id) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if remove(&mut result, id) {
            Some(result.into_owned())
        } else {
            None
        }
    }

    pub fn remove_all<T: Into<Id>, I: IntoIterator<Item = T>>(&self, iterable: I) -> Self {
        let mut result = self.clone();
        result.remove_all_mut(iterable);
        result
    }

    pub fn try_remove_all<T: Into<Id>, I: IntoIterator<Item = T>>(&self, iterable: I) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if remove_all(&mut result, iterable) {
            Some(result.into_owned())
        } else {
            None
        }
    }

    pub fn disconnect(&self, source: Id, sink: Id) -> Self {
        let mut result = self.clone();
        result.disconnect_mut(source, sink);
        result
    }

    pub fn try_disconnect(&self, source: Id, sink: Id) -> Option<Self> {
        let mut result = Cow::Borrowed(self);
        if result.has_vertex(source) && result.has_vertex(sink) {
            result.to_mut().disconnect_mut(source, sink);
            Some(result.into_owned())
        } else {
            None
        }
    }

    pub fn remove_mut(&mut self, id: Id) -> bool {
        if self.has_vertex(id) {
            self.remove_mut_no_inc(id);
            self.idgen.next_gen();
            true
        } else {
            false
        }
    }

    pub fn remove_all_mut<T: Into<Id>, I: IntoIterator<Item = T>>(&mut self, iter: I) -> bool {
        let changed = self.remove_all_mut_no_inc(iter);
        if changed {
            self.idgen.next_gen();
        };
        changed
    }

    fn remove_all_mut_no_inc<T: Into<Id>, I: IntoIterator<Item = T>>(
        &mut self,
        iterable: I,
    ) -> bool {
        iterable.into_iter().fold(false, |changed, into_id| {
            self.try_remove_mut_no_inc(into_id.into()) || changed
        })
    }

    fn try_remove_mut_no_inc(&mut self, id: Id) -> bool {
        if self.has_vertex(id) {
            self.remove_mut_no_inc(id);
            true
        } else {
            false
        }
    }

    fn remove_mut_no_inc(&mut self, id: Id) {
        self.guts.set_mut(id.get_index(), None);
        self.disconnect_all_inc_mut(id);
    }

    pub fn disconnect_mut(&mut self, source: Id, sink: Id) -> bool {
        self[source].disconnect(sink)
    }

    pub fn try_disconnect_mut(&mut self, source: Id, sink: Id) -> bool {
        self.get_mut(source).map_or(false, |v| v.disconnect(sink))
    }

    fn disconnect_all_inc_mut(&mut self, sink: Id) -> bool {
        let inc: Vec<Id> = self.vertices_with_edge_to(sink).cloned().collect();
        inc.into_iter().fold(false, |init, source| {
            self.disconnect_mut(source, sink) || init
        })
    }
}

impl<V, E> Index<Id> for Graph<V, E> {
    type Output = Vertex<V, E>;

    fn index(&self, id: Id) -> &Vertex<V, E> {
        match self.guts.get(id.get_index()) {
            Some(Some(vertex)) if vertex.same_id(id) => vertex,
            Some(Some(_)) => panic!("The Id {:?} is of an invalid generation. It does not correspond to any vertices in this graph.", id),
            Some(None) => panic!("No vertex found for Id {:?}. It has likely been removed from the graph.", id),
            None => panic!("No vertex found for Id {:?}. The Id either comes from a chlid or another graph family.", id),
        }
    }
}

impl<V: Clone, E> IndexMut<Id> for Graph<V, E> {
    fn index_mut(&mut self, id: Id) -> &mut Vertex<V, E> {
        match self.guts.get_mut(id.get_index()) {
            Some(Some(vertex)) if vertex.same_id(id) => vertex,
            Some(Some(_)) => panic!("The Id {:?} is of an invalid generation. It does not correspond to any vertices in this graph", id),
            Some(None) => panic!("No vertex found for Id {:?}. It has likely been removed from the graph.", id),
            None => panic!("No vertex found for Id {:?}. The Id either comes from a chlid or another graph family.", id),
        }
    }
}

impl<V, E> Index<(Id,)> for Graph<V, E> {
    type Output = V;

    fn index(&self, id: (Id,)) -> &V {
        self[id.0].get_data()
    }
}

impl<V: Clone, E> IndexMut<(Id,)> for Graph<V, E> {
    fn index_mut(&mut self, id: (Id,)) -> &mut V {
        self[id.0].get_data_mut()
    }
}

impl<V, E> Index<(Id, Id)> for Graph<V, E> {
    type Output = E;

    fn index(&self, ids: (Id, Id)) -> &E {
        let (source, sink) = ids;
        self[source].index(sink)
    }
}

impl<V: Clone, E: Clone> IndexMut<(Id, Id)> for Graph<V, E> {
    fn index_mut(&mut self, ids: (Id, Id)) -> &mut E {
        let (source, sink) = ids;
        self[source].index_mut(sink)
    }
}

fn remove<'a, V: Clone, E: Clone>(cow: &mut Cow<'a, Graph<V, E>>, id: Id) -> bool {
    if cow.has_vertex(id) {
        cow.to_mut().remove_mut_no_inc(id);
        true
    } else {
        false
    }
}

fn remove_all<'a, V: Clone, E: Clone, T: Into<Id>, I: IntoIterator<Item = T>>(
    cow: &mut Cow<'a, Graph<V, E>>,
    iterable: I,
) -> bool {
    iterable.into_iter().fold(false, |changed, into_id| {
        remove(cow, into_id.into()) || changed
    })
}

type GutsIter<'a, V, E> = <&'a GraphInternal<V, E> as IntoIterator>::IntoIter;
type VertexDeref<'a, V, E> = fn(&'a Option<Vertex<V, E>>) -> Option<&'a Vertex<V, E>>;

impl<'a, V, E> IntoIterator for &'a Graph<V, E> {
    type Item = &'a Vertex<V, E>;
    type IntoIter = FilterMap<GutsIter<'a, V, E>, VertexDeref<'a, V, E>>;

    fn into_iter(self) -> Self::IntoIter {
        self.guts.iter().filter_map(|v_opt| v_opt.as_ref())
    }
}
