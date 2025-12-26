use crate::instance::node::Node;

#[derive(Debug, Clone)]
pub struct EdgeDataMatrixSym<Data> {
    pub data: Vec<Data>,
    pub dimension: usize,
}

impl<Data: Copy> EdgeDataMatrixSym<Data> {
    pub fn new_from_data(distance_data: Vec<Data>, dimension: usize) -> Self {
        assert_eq!(distance_data.len(), dimension * (dimension + 1) / 2);
        Self {
            data: distance_data,
            dimension,
        }
    }

    #[inline(always)]
    pub fn get_data(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_column_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn set_data(&mut self, from: Node, to: Node, data: Data) {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index] = data;
    }

    pub fn restrict_to_first_n<'a>(&'a self, n: usize) -> RestrictedDataMatrixSym<'a, Data> {
        RestrictedDataMatrixSym {
            data: &self.data[0..(n * (n + 1)) / 2],
            dimension: n,
        }
    }

    pub fn to_non_symmetric(&self) -> EdgeDataMatrix<Data> {
        let dimension = self.dimension;
        let mut data = vec![self.data[0].clone(); dimension * dimension];
        for row in 0..dimension {
            for column in 0..row {
                let value = self.get_data_from_bigger(Node(row), Node(column));
                data[row * self.dimension + column] = value.clone();
                data[column * self.dimension + row] = value;
            }
        }
        EdgeDataMatrix {
            data,
            dimension: self.dimension,
        }
    }
}

impl<Data: Clone> EdgeDataMatrixSym<Data> {
    pub fn new_from_dimension_with_value(dimension: usize, value: Data) -> Self {
        let size = (dimension * (dimension + 1)) / 2;
        Self {
            data: vec![value; size],
            dimension,
        }
    }
}

impl<Data: Default + Clone + Copy> EdgeDataMatrixSym<Data> {
    pub fn slow_new_from_distance_function(
        dimension: usize,
        mut distance_function: impl FnMut(Node, Node) -> Data,
    ) -> Self {
        let mut res = EdgeDataMatrixSym::new_from_dimension_with_value(dimension, Data::default());
        for row in 0..dimension {
            for column in 0..row {
                let distance = distance_function(Node(row), Node(column));
                res.set_data(Node(row), Node(column), distance);
            }
        }
        res
    }
}

pub struct RestrictedDataMatrixSym<'a, Data> {
    pub data: &'a [Data],
    pub dimension: usize,
}

impl<'a, Data: Copy> RestrictedDataMatrixSym<'a, Data> {
    #[inline(always)]
    pub fn get_data(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        debug_assert!(
            index < self.data.len(),
            "Index out of bounds in RestrictedDataMatrixSym: index {}, data length {}, from {:?}, \
             to {:?}",
            index,
            self.data.len(),
            from,
            to
        );
        self.data[index]
    }

    #[inline(always)]
    pub fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    #[inline(always)]
    pub fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_column_bigger(from.0, to.0);
        self.data[index]
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

#[derive(Debug, Clone)]
/// Row major full matrix representation of edge data.
pub struct EdgeDataMatrix<Data> {
    pub data: Vec<Data>,
    pub dimension: usize,
}

impl<Data> EdgeDataMatrix<Data> {
    pub fn new(data: Vec<Data>, dimension: usize) -> EdgeDataMatrix<Data> {
        debug_assert_eq!(data.len(), dimension * dimension);
        EdgeDataMatrix { data, dimension }
    }
}

impl<Data: Clone> EdgeDataMatrix<Data> {
    pub fn new_from_dimension_with_value(dimension: usize, value: Data) -> Self {
        let size = dimension * dimension;
        Self {
            data: vec![value; size],
            dimension,
        }
    }
}

impl<Data: Copy> EdgeDataMatrix<Data> {
    /// Access the data.
    #[inline(always)]
    pub fn get_data(&self, from: Node, to: Node) -> Data {
        let index = from.0 * self.dimension + to.0;
        self.data[index]
    }

    /// Access the data in a way that allows for faster sequential access when iterating over 'to'
    /// nodes.
    #[inline(always)]
    pub fn get_data_to_seq(&self, from: Node, to: Node) -> Data {
        let index = from.0 * self.dimension + to.0;
        self.data[index]
    }

    /// Access the data in a way that allows for faster sequential access when iterating over 'from'
    /// nodes.
    #[inline(always)]
    pub fn get_data_from_seq(&self, from: Node, to: Node) -> Data {
        self.get_data_to_seq(to, from)
    }

    /// Set data.
    #[inline(always)]
    pub fn set_data(&mut self, from: Node, to: Node, data: Data) {
        let index = from.0 * self.dimension + to.0;
        self.data[index] = data;
        let index = to.0 * self.dimension + from.0;
        self.data[index] = data;
    }

    /// Get the adjacency list for a given 'from' node.
    #[inline(always)]
    pub fn get_adjacency_list(&self, from: Node) -> &[Data] {
        let start_index = from.0 * self.dimension;
        &self.data[start_index..start_index + self.dimension]
    }

    /// Split the matrix into a zero row and a zero-removed matrix.
    pub fn split_first_row<'a>(&'a self) -> (&'a [Data], EdgeDataMatrixZeroRemoved<'a, Data>) {
        let zero_row = &self.data[0..self.dimension];
        let zero_removed = EdgeDataMatrixZeroRemoved {
            data: &self.data[self.dimension..],
            dimension: self.dimension,
        };
        (zero_row, zero_removed)
    }
}

impl<Data: Default + Clone + Copy> EdgeDataMatrix<Data> {
    pub fn slow_new_from_distance_function(
        dimension: usize,
        mut distance_function: impl FnMut(Node, Node) -> Data,
    ) -> Self {
        let mut res = EdgeDataMatrix::new_from_dimension_with_value(dimension, Data::default());
        for row in 0..dimension {
            for column in 0..row {
                let distance = distance_function(Node(row), Node(column));
                res.set_data(Node(row), Node(column), distance);
            }
        }
        res
    }
}

/// Row major full matrix representation of edge data with zero row/column removed.
///
/// I.e. a (n-1) x n matrix where row 0 corresponds to node 1, row 1 to node 2, ..., row n-1 to node
/// n.
pub struct EdgeDataMatrixZeroRemoved<'a, Data> {
    pub data: &'a [Data],
    dimension: usize,
}

impl<'a, Data: Copy> EdgeDataMatrixZeroRemoved<'a, Data> {
    /// Get the adjusted dimension (i.e., n-1 if original dimension is n).
    pub fn dimension_adjusted(&self) -> usize {
        self.dimension - 1
    }

    /// Get the total dimension (i.e., n if original dimension is n).
    pub fn dimension_total(&self) -> usize {
        self.dimension
    }

    /// Get the adjacency list for a given 'from' node. Assumes `from` is not node 0, i.e., starts
    /// at 1. That is, takes into account that the zero row/column has been removed.
    #[inline(always)]
    pub fn get_adjacency_list(&self, from: Node) -> &[Data] {
        let start_index = (from.0 - 1) * self.dimension;
        &self.data[start_index..start_index + self.dimension]
    }
}
