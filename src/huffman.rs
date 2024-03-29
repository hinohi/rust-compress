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
    fn make_encoder(&self, map: &mut Vec<BitVec>, current: &mut Vec<bool>) {
        if let Some(value) = self.value {
            map[value] = current.as_slice().into();
        } else {
            if let Some(ref child) = self.left {
                current.push(true);
                child.make_encoder(map, current);
                current.pop();
            }
            if let Some(ref child) = self.right {
                current.push(false);
                child.make_encoder(map, current);
                current.pop();
            }
        }
    }

    fn make_decoder(&self, map: &mut Vec<DecoderNode>) {
        if let Some(value) = self.value {
            map.push(DecoderNode::Value(value));
        } else {
            let idx = map.len();
            map.push(DecoderNode::Jump(0));
            if let Some(ref child) = self.left {
                child.make_decoder(map);
            }
            map[idx] = DecoderNode::Jump(map.len());
            if let Some(ref child) = self.right {
                child.make_decoder(map);
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
        self.nodes.make_encoder(&mut map, &mut Vec::new());
        HuffmanEncoder { map }
    }

    pub fn decoder(&self) -> HuffmanDecoder {
        let mut map = Vec::with_capacity(self.elements * 2 + 1);
        self.nodes.make_decoder(&mut map);
        HuffmanDecoder { map }
    }
}

#[derive(Clone, Debug)]
pub struct HuffmanEncoder {
    map: Vec<BitVec>,
}

impl HuffmanEncoder {
    pub fn encode(&self, value: usize) -> &BitVec {
        self.map.get(value).unwrap()
    }
}

#[derive(Clone, Debug)]
enum DecoderNode {
    Jump(usize),
    Value(usize),
}

#[derive(Clone, Debug)]
pub struct HuffmanDecoder {
    map: Vec<DecoderNode>,
}

impl HuffmanDecoder {
    pub fn decode<I>(&self, input: &mut I) -> usize
    where
        I: Iterator<Item = bool>,
    {
        let mut idx = 0;
        while let DecoderNode::Jump(right) = self.map[idx] {
            if input.next().unwrap() {
                idx += 1;
            } else {
                idx = right;
            }
        }
        match self.map[idx] {
            DecoderNode::Value(value) => value,
            _ => unreachable!(),
        }
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

    #[test]
    fn encode_2() {
        let tree = HuffmanTree::new(&[1, 2, 4, 8]);
        let encoder = tree.encoder();
        assert_eq!(encoder.encode(0).len(), 3);
        assert_eq!(encoder.encode(1).len(), 3);
        assert_eq!(encoder.encode(2).len(), 2);
        assert_eq!(encoder.encode(3).len(), 1);
    }

    #[test]
    fn encode_decode() {
        let tree = HuffmanTree::new(&[10, 100, 20, 50, 60, 10]);
        let encoder = tree.encoder();
        let decoder = tree.decoder();
        assert_eq!(decoder.decode(&mut encoder.encode(0).iter()), 0);
        assert_eq!(decoder.decode(&mut encoder.encode(1).iter()), 1);
        assert_eq!(decoder.decode(&mut encoder.encode(2).iter()), 2);
        assert_eq!(decoder.decode(&mut encoder.encode(3).iter()), 3);
        assert_eq!(decoder.decode(&mut encoder.encode(4).iter()), 4);
        assert_eq!(decoder.decode(&mut encoder.encode(5).iter()), 5);
    }
}
