use std::fmt::{Debug, Error, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A generational ID for some peice of data. Conceptually, you can think of it as a pointer
/// that can only be created pointing to valid data (no nulls), and automatically protects against use-after-free.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    index: usize,
    generation: usize,
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} gen{}", self.index, self.generation)
    }
}

impl<'a> From<&'a Id> for Id {
    fn from(other: &'a Id) -> Self {
        *other
    }
}

impl Id {
    /// Gets the generation of this ID. Should only be needed for debug/testing.
    /// Normal use cases should only care if two Ids generations are equal, not
    /// what that generation is, since the generation can be nondeterministic.
    #[must_use]
    #[cfg(test)]
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Allows this crate to get the index out of an Id.
    /// Considered using Into<usize>, but that makes users able to use the conversion.
    #[must_use]
    pub(crate) fn get_index(&self) -> usize {
        self.index
    }
}

static GENERATION: AtomicUsize = AtomicUsize::new(0);

/// Keeps track of a current generation and creates new [Id](struct.Id.html)s from that generation.
/// The generation of two IdGen instances will never be the same (that guarantee is thread-safe).
/// However, this means that the generation has no guaranteed starting point or step, so you shouldn't depend
/// on _specific_ generations, only check for equality.
pub(crate) struct IdGen {
    current_gen: usize,
}

impl Debug for IdGen {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "gen {}", self.current_gen)
    }
}

impl IdGen {
    /// Creates a new IdGen. The generation will not conflict with any other IdGen already instantiated or instantiated in the future
    #[must_use]
    pub(crate) fn new() -> Self {
        IdGen {
            current_gen: GENERATION.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Creates a new id with the given index for the current generation
    #[must_use]
    pub(crate) fn create_id(&self, index: usize) -> Id {
        Id {
            index,
            generation: self.current_gen,
        }
    }

    /// Increments the current generation by some number. The current generation after calling this function is guaranteed
    /// to be strictly greater than it was before, but _how much greater_ is undefined.
    pub(crate) fn next_gen(&mut self) {
        self.current_gen = GENERATION.fetch_add(1, Ordering::Relaxed);
    }

    /// Gets the IdGen's current generation
    #[cfg(test)]
    pub(crate) fn generation(&self) -> usize {
        self.current_gen
    }
}

impl Clone for IdGen {
    fn clone(&self) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_gen() {
        let mut a = IdGen::new();
        let a_gen = a.current_gen;
        let b = a.clone();

        assert_eq!(a_gen, a.current_gen);
        assert_lt!(a.current_gen, b.current_gen);

        a.next_gen();
        assert_lt!(a_gen, a.current_gen);
    }

    #[test]
    fn test_gen_of_ids() {
        let a = IdGen::new();
        let a_first = a.create_id(1);

        let mut b = a.clone();
        b.next_gen();
        let b_first = b.create_id(1);

        let mut c = b.clone();
        c.next_gen();
        let c_first = c.create_id(1);

        assert_same_index(&a_first, &b_first);
        assert_same_index(&b_first, &c_first);

        println!(
            "Generators and ids:\n\ta = {} -> {:?}, \tb = {} -> {:?}, \tc = {} -> {:?}",
            a.current_gen, a_first, b.current_gen, c_first, c.current_gen, c_first
        );

        let a1 = a.create_id(1);
        let b1 = b.create_id(1);
        let c1 = c.create_id(1);

        assert_eq!(a1.generation, a.current_gen);
        assert_eq!(b1.generation, b.current_gen);
        assert_eq!(c1.generation, c.current_gen);

        assert_lt!(a1.generation, b1.generation);
        assert_lt!(b1.generation, c1.generation);
    }

    fn assert_same_index(id1: &Id, id2: &Id) {
        let a: usize = id1.get_index();
        let b: usize = id2.get_index();

        assert_eq!(a, b);
    }
}
