use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::bit_vec::BitVec;

#[derive(Clone, Debug)]
struct Node {
    count: u128,
    value: Option<usize>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.count == other.count
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // reverse the order, to pop minimum value from BinaryHeap
        other.count.cmp(&self.count)
    }
}

impl Node {
    fn walk(&self, map: &mut Vec<BitVec>) {
        self.walk_node(map, &mut Vec::new());
    }

    fn walk_node(&self, map: &mut Vec<BitVec>, current: &mut Vec<bool>) {
        if let Some(value) = self.value {
            map[value] = current.as_slice().into();
        } else {
            if let Some(ref child) = self.left {
                current.push(true);
                child.walk_node(map, current);
                current.pop();
            }
            if let Some(ref child) = self.right {
                current.push(false);
                child.walk_node(map, current);
                current.pop();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct HuffmanTree {
    elements: usize,
    nodes: Node,
}

impl HuffmanTree {
    pub fn new(counts: &[u128]) -> HuffmanTree {
        assert!(counts.len() > 1);
        let mut heap = BinaryHeap::with_capacity(counts.len());
        for (value, count) in counts.iter().enumerate() {
            heap.push(Node {
                count: *count,
                value: Some(value),
                left: None,
                right: None,
            });
        }
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            heap.push(Node {
                count: left.count + right.count,
                value: None,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            });
        }
        HuffmanTree {
            elements: counts.len(),
            nodes: heap.pop().unwrap(),
        }
    }

    pub fn encoder(&self) -> HuffmanEncoder {
        let mut map = Vec::with_capacity(self.elements);
        for _ in 0..self.elements {
            map.push(BitVec::new());
        }
        self.nodes.walk(&mut map);
        HuffmanEncoder { map }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct HuffmanEncoder {
    map: Vec<BitVec>,
}

impl HuffmanEncoder {
    pub fn encode(&self, value: usize) -> &BitVec {
        self.map.get(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_uniform() {
        let tree = HuffmanTree::new(&[1, 1, 1, 1]);
        let encoder = tree.encoder();
        assert_eq!(encoder.encode(0).len(), 2);
        assert_eq!(encoder.encode(1).len(), 2);
        assert_eq!(encoder.encode(2).len(), 2);
        assert_eq!(encoder.encode(3).len(), 2);
    }
}
