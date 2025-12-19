#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub struct WeightedEdge {
    pub cost: u32,
    pub from: usize,
    pub to: usize,
}

impl WeightedEdge {
    pub fn new(cost: u32, from: usize, to: usize) -> Self {
        Self { cost, from, to }
    }

    pub fn to_edge(&self) -> Edge {
        Edge {
            from: self.from,
            to: self.to,
        }
    }
}

impl PartialEq for WeightedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for WeightedEdge {}

impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
