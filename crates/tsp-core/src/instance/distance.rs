use crate::instance::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Distance(pub i32);

pub trait DistanceMatrix {
    fn get_distance(&self, from: Node, to: Node) -> Distance;
    fn get_distance_from_bigger(&self, from: Node, to: Node) -> Distance {
        self.get_distance(from, to)
    }
    fn get_distance_to_bigger(&self, from: Node, to: Node) -> Distance {
        self.get_distance(from, to)
    }

    fn dimension(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct DistanceMatrixSymmetric {
    pub data: Vec<Distance>,
    pub dimension: usize,
}

impl DistanceMatrixSymmetric {
    pub fn new_from_data(distance_data: Vec<Distance>, dimension: usize) -> Self {
        assert_eq!(distance_data.len(), dimension * (dimension + 1) / 2);
        Self {
            data: distance_data,
            dimension,
        }
    }

    pub fn new_from_dimension_with_value(dimension: usize, value: Distance) -> Self {
        let size = (dimension * (dimension + 1)) / 2;
        Self {
            data: vec![value; size],
            dimension,
        }
    }

    pub fn slow_new_from_distance_function(
        dimension: usize,
        mut distance_function: impl FnMut(Node, Node) -> Distance,
    ) -> Self {
        let mut res =
            DistanceMatrixSymmetric::new_from_dimension_with_value(dimension, Distance(0));
        for row in 0..dimension {
            for column in 0..row {
                let distance = distance_function(Node(row), Node(column));
                res.set_distance(Node(row), Node(column), distance);
            }
        }
        res
    }

    #[inline(always)]
    pub fn get_distance(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_distance_from_bigger(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_distance_to_bigger(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry_column_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn set_distance(&mut self, from: Node, to: Node, distance: Distance) {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index] = distance;
    }

    pub fn restrict_to_first_n<'a>(&'a self, n: usize) -> RestrictedDistanceMatrixSymmetric<'a> {
        RestrictedDistanceMatrixSymmetric {
            data: &self.data[0..(n * (n - 1)) / 2],
            dimension: n,
        }
    }
}

impl DistanceMatrix for DistanceMatrixSymmetric {
    fn get_distance(&self, from: Node, to: Node) -> Distance {
        self.get_distance(from, to)
    }

    fn get_distance_from_bigger(&self, from: Node, to: Node) -> Distance {
        self.get_distance_to_bigger(from, to)
    }

    fn get_distance_to_bigger(&self, from: Node, to: Node) -> Distance {
        self.get_distance_from_bigger(from, to)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

pub struct RestrictedDistanceMatrixSymmetric<'a> {
    pub data: &'a [Distance],
    pub dimension: usize,
}

impl<'a> RestrictedDistanceMatrixSymmetric<'a> {
    #[inline(always)]
    pub fn get_distance(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_distance_from_bigger(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_distance_to_bigger(&self, from: Node, to: Node) -> Distance {
        let index = get_lower_triangle_matrix_entry_column_bigger(from.0, to.0);
        self.data[index]
    }
}

impl<'a> DistanceMatrix for RestrictedDistanceMatrixSymmetric<'a> {
    fn get_distance(&self, from: Node, to: Node) -> Distance {
        self.get_distance(from, to)
    }

    fn get_distance_to_bigger(&self, column: Node, row: Node) -> Distance {
        self.get_distance_to_bigger(column, row)
    }

    fn get_distance_from_bigger(&self, row: Node, column: Node) -> Distance {
        self.get_distance_from_bigger(row, column)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

#[inline(always)]
/// Computes the index in a vec-flattened lower-(left-)triangular matrix.
pub fn get_lower_triangle_matrix_entry(row: usize, column: usize) -> usize {
    if row > column {
        get_lower_triangle_matrix_entry_row_bigger(row, column)
    } else {
        get_lower_triangle_matrix_entry_column_bigger(row, column)
    }
}

#[inline(always)]
/// Computes the index in a vec-flattened lower-(left-)triangular matrix assuming row >= column.
pub fn get_lower_triangle_matrix_entry_row_bigger(row: usize, column: usize) -> usize {
    (row * (row + 1)) / 2 + column
}

#[inline(always)]
/// Computes the index in a vec-flattened lower-(left-)triangular matrix assuming column >= row.
pub fn get_lower_triangle_matrix_entry_column_bigger(row: usize, column: usize) -> usize {
    get_lower_triangle_matrix_entry_row_bigger(column, row)
}
