#![allow(soft_unstable)]
#![feature(test)]

use std::collections::HashMap;
use std::hash::Hash;
use std::vec::Vec;
use std::option::Option;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Index (usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    parent: Index,
    size: usize,
}

pub struct UnionFind<T> {
    to_index: HashMap<T, Index>,
    from_index: Vec<T>,
    nodes: Vec<Node>,
}

impl <T: Copy + Eq + Hash + PartialOrd> UnionFind<T> {
    pub fn new() -> Self {
        Self {
            to_index: HashMap::new(),
            from_index: Vec::with_capacity(1),
            nodes: Vec::with_capacity(1),
        }
    }

    pub fn insert(&mut self, x: T) {
        let index = Index(self.nodes.len());
        self.nodes.push(Node { parent: index, size: 1 });
        self.to_index.insert(x, index);
        self.from_index.push(x);
    }

    pub fn unions(&mut self, xs: &[T]) -> bool {
        if xs.len() < 2 {
            true
        } else {
            let head = xs[0];
            let mut iter = xs.iter();
            iter.next(); // skip the head
            for x in iter {
                let b = self.union(head, *x);
                if !b {
                    return false
                }
            }
            true
        }
    }

    pub fn union(&mut self, x: T, y: T) -> bool {
        match self.union_internal(x, y) {
            None => false,
            Some(_) => true,
        }
    }

    fn union_internal(&mut self, x: T, y: T) -> Option<()> {
        if x > y {
            self.union_internal(y, x)
        } else {
            let x_root = self.find_get(x)?;
            let y_root = self.find_get(y)?;
            if x_root != y_root {
                let x_root_size = self.nodes[x_root.0].size;
                let y_root_size = self.nodes[y_root.0].size;
                if x_root_size < y_root_size {
                    self.nodes.swap(x_root.0, y_root.0);
                }
                self.nodes[y_root.0].parent = x_root;
                self.nodes[x_root.0].size = x_root_size + y_root_size;
                Some(())
            } else {
                Some(())
            }
        }
    }


    pub fn find(&mut self, x: T) -> Option<T> {
        let class_index = self.find_get(x)?;
        match self.from_index.get(class_index.0) {
            None => None,
            Some(t) => Some(*t),
        }
    }

    fn find_get(&mut self, x: T) -> Option<Index> {
        let elem_index = HashMap::get(&self.to_index, &x)?;
        find_index(&mut self.nodes, *elem_index)
    }
}

fn find_index(nodes: &mut [Node], x: Index) -> Option<Index> {
    let p = nodes[x.0].parent;
    if p != x {
        // parent of parent of x
        let pp = nodes[p.0].parent;

        nodes[p.0].parent = pp;
        find_index(nodes, p)
    } else {
        Some(x)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;
    use std::slice::Chunks;

    #[test]
    fn it_works() {
        let mut u = UnionFind::new();
        for i in 1..9 {
            u.insert(i);
        }
        u.unions(&[1,2,5,6,8]);
        u.unions(&[3,4]);
        u.unions(&[7]);

        assert_eq!(u.find(6), Some(1));
        assert_eq!(u.find(3), Some(3));
        assert_eq!(u.find(7), Some(7));
    }

    #[bench]
    fn bench_create(b: &mut Bencher) {
        b.iter(|| {
            let mut u = UnionFind::new();

            let rng = 0i32 .. 100_000;

            for i in rng {
              u.insert(i);
            }

            for chunk in 0 .. 2000 {
                for elem in 0 .. 500 {
                    u.union(chunk * 500, chunk * 500 + elem);
                }
            }
        });
    }


}
