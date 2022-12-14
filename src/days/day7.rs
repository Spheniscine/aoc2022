use std::{rc::Rc, ops::{Index, IndexMut}};
use shash::SHash;

use crate::aoc_base::Day;

pub struct Day7;

type IndexMap<K, V> = indexmap::IndexMap<K, V, SHash>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeKey(u32);
impl NodeKey {
    const NULL: NodeKey = NodeKey(!0);
    pub fn is_null(self) -> bool { self == NodeKey::NULL }
    pub fn is_not_null(self) -> bool { self != NodeKey::NULL }
}
const ROOT: NodeKey = NodeKey(0);

#[derive(Clone, Debug)]
pub struct Node {
    name: Rc<str>,
    size: i64,
    parent: NodeKey,
    resource: Resource
}

impl Node {
    fn new_dir(name: Rc<str>, parent: NodeKey) -> Self {
        Self {
            name,
            size: 0,
            parent,
            resource: Resource::Dir { contents: <_>::default() },
        }
    }
    fn new_file(name: Rc<str>, size: i64, parent: NodeKey) -> Self {
        Self {
            name,
            size,
            parent,
            resource: Resource::File,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Resource {
    Dir {contents: IndexMap<Rc<str>, NodeKey>}, File
}

#[derive(Clone, Debug)]
pub struct System {
    nodes: Vec<Node>
}
impl System {
    fn new() -> Self {
        Self { nodes: vec![Node::new_dir("".into(), NodeKey::NULL)] }
    }
    fn insert_node(&mut self, node: Node) -> NodeKey {
        let res = NodeKey(self.nodes.len() as _);

        match &mut self[node.parent].resource {
            Resource::Dir { contents } => { contents.insert(node.name.clone(), res); }
            Resource::File => { panic!("Parent can't be a file"); }
        }
        self.nodes.push(node);

        res
    }

    fn cd(&mut self, curr: NodeKey, name: &str) -> NodeKey {
        match &mut self[curr].resource {
            Resource::Dir { contents } => { 
                if let Some(v) = contents.get(name) {
                    *v
                } else {
                    let node = Node::new_dir(name.into(), curr);
                    self.insert_node(node)
                }
            }
            Resource::File => { panic!("Can't cd from a file"); }
        }
    }

    fn calc_sizes(&mut self) {
        let mut stk = vec![(ROOT, true)];
        while let Some((u, fs)) = stk.pop() {
            let mut size = 0;
            match &self[u].resource {
                Resource::Dir { contents } => {
                    if fs {
                        stk.push((u, false));
                        for &v in contents.values() {
                            stk.push((v, true));
                        }
                    } else {
                        for &v in contents.values() {
                            size += self[v].size;
                        }
                    }
                }
                Resource::File => (),
            }
            if !fs { self[u].size = size; }
        }
    }
}
impl Index<NodeKey> for System {
    type Output = Node;

    fn index(&self, index: NodeKey) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}
impl IndexMut<NodeKey> for System {
    fn index_mut(&mut self, index: NodeKey) -> &mut Self::Output {
        &mut self.nodes[index.0 as usize]
    }
}

impl Day for Day7 {
    type Parsed = System;

    type Output1 = i64;

    type Output2 = i64;

    fn num() -> usize {
        7
    }

    fn title() -> &'static str {
        "No Space Left On Device"
    }

    fn parse(input: &str) -> Self::Parsed {
        let mut lines = input.lines().peekable();

        let mut sys = System::new();
        let mut curr = ROOT;

        while let Some(ln) = lines.next() {
            if let Some(arg) = ln.strip_prefix("$ cd ") {
                if arg == "/" {
                    curr = ROOT;
                } else if arg == ".." {
                    curr = sys[curr].parent;
                } else {
                    curr = sys.cd(curr, arg);
                }
            } else if ln == "$ ls" {
                while let Some(&ln) = lines.peek() {
                    if ln.starts_with("$") { break; }
                    lines.next();

                    let mut sp = ln.split(' ');
                    let a = sp.next().unwrap();
                    let b = sp.next().unwrap();

                    if a == "dir" {
                        sys.cd(curr, b);
                    } else {
                        let file = Node::new_file(b.into(), a.parse().unwrap(), curr);
                        sys.insert_node(file);
                    }
                }
            } else {
                panic!("Unknown command {}", ln)
            }
        }

        sys.calc_sizes();
        sys
    }

    fn part1(sys: &Self::Parsed) -> Self::Output1 {
        sys.nodes.iter().map(|node| {
            if matches!(node.resource, Resource::Dir{..}) && node.size <= 100_000 { node.size } else { 0 }
        }).sum()
    }

    fn part2(sys: &Self::Parsed) -> Self::Output2 {
        let need = 30000000 - (70000000 - sys[ROOT].size);

        sys.nodes.iter().filter_map(|node| {
            if matches!(node.resource, Resource::Dir{..}) && node.size >= need { Some(node.size) } else { None }
        }).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_base::InputSource;
    use super::*;

    #[test]
    fn gmail() {
        Day7::test(InputSource::gmail(7), Some(1513699), Some(7991939))
    }
}