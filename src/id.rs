use std::cmp::Ordering;
use std::fmt::{ Debug, Error, Formatter };

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Id {
    index: usize,
    generation: usize,
}

impl<'a> Into<usize> for &'a Id {
    fn into(self) -> usize {
        self.index
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.index != other.index {
            None
        } else {
            self.generation.partial_cmp(&other.generation)
        }
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} gen{}", self.index, self.generation)
    }
}

#[derive(Clone, Debug)]
pub struct IdGen {
    generation: usize,
}

impl IdGen {
    pub fn new() -> Self {
        IdGen { generation: 0 }
    }

    pub fn create_id(&self, index: usize) -> Id {
        Id {
            index,
            generation: self.generation,
        }
    }

    pub fn next_gen(&mut self) {
        self.generation += 1;
    }

    #[cfg(test)]
    fn get_gen(&self) -> usize {
        self.generation
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_gen() {
        let a = IdGen::new();
        let mut b = a.clone();
        b.next_gen();
        let mut c = b.clone();
        c.next_gen();

        assert!(a.get_gen() < b.get_gen());
        assert!(b.get_gen() < c.get_gen());
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

        let a_gen = a.get_gen();
        let b_gen = b.get_gen();
        let c_gen = c.get_gen();

        println!("a = {},  b = {}, c = {}", a_gen, b_gen, c_gen);

        let a1 = a.create_id(1);
        let b1 = b.create_id(1);
        let c1 = c.create_id(1);

        assert_same_index(&a1, &b1);
        assert_same_index(&b1, &c1);

        assert!(a1 < b1);
        assert!(b1 < c1);
    }

    fn assert_same_index(id1: &Id, id2: &Id) {
        let a: usize = id1.into();
        let b: usize = id2.into();

        assert_eq!(a, b);
    }
}