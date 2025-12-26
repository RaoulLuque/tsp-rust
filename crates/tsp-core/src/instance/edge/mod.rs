use crate::instance::{edge::distance::Distance, node::Node};

pub mod data;
pub mod distance;

/// An undirected edge between two nodes.
#[derive(Debug, Clone, Copy)]
pub struct UnEdge {
    pub from: Node,
    pub to: Node,
}

impl UnEdge {
    pub fn new(from: Node, to: Node) -> Self {
        Self { from, to }
    }
}

impl PartialEq for UnEdge {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

impl Eq for UnEdge {}

impl PartialOrd for UnEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (min_self, max_self) = if self.from <= self.to {
            (self.from, self.to)
        } else {
            (self.to, self.from)
        };
        let (min_other, max_other) = if other.from <= other.to {
            (other.from, other.to)
        } else {
            (other.to, other.from)
        };

        match min_self.cmp(&min_other) {
            std::cmp::Ordering::Equal => max_self.cmp(&max_other),
            ord => ord,
        }
    }
}

#[derive(Debug)]
/// An undirected edge with an inverse weight for use in a max-heap.
///
/// That is, when comparing two edges, the one with the lower cost is considered greater.
pub struct InvWeightUnEdge {
    pub cost: Distance,
    pub from: Node,
    pub to: Node,
}

impl InvWeightUnEdge {
    pub fn new(cost: Distance, from: Node, to: Node) -> Self {
        Self { cost, from, to }
    }

    pub fn to_edge(&self) -> UnEdge {
        UnEdge {
            from: self.from,
            to: self.to,
        }
    }
}

impl PartialEq for InvWeightUnEdge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for InvWeightUnEdge {}

impl PartialOrd for InvWeightUnEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for InvWeightUnEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
