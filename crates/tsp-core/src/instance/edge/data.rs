use std::marker::PhantomData;

use crate::instance::node::Node;

pub trait EdgeDataMatrix<Data> {
    fn get_data(&self, from: Node, to: Node) -> Data;
    fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        self.get_data(from, to)
    }
    fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        self.get_data(from, to)
    }

    fn dimension(&self) -> usize;
}

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

impl<Data: Copy> EdgeDataMatrix<Data> for EdgeDataMatrixSym<Data> {
    fn get_data(&self, from: Node, to: Node) -> Data {
        EdgeDataMatrixSym::get_data(self, from, to)
    }

    fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        EdgeDataMatrixSym::get_data_from_bigger(self, from, to)
    }

    fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        EdgeDataMatrixSym::get_data_to_bigger(self, from, to)
    }

    fn dimension(&self) -> usize {
        self.dimension
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

impl<'a, Data: Copy> EdgeDataMatrix<Data> for RestrictedDataMatrixSym<'a, Data> {
    fn get_data(&self, from: Node, to: Node) -> Data {
        RestrictedDataMatrixSym::get_data(self, from, to)
    }

    fn get_data_to_bigger(&self, column: Node, row: Node) -> Data {
        RestrictedDataMatrixSym::get_data_to_bigger(self, column, row)
    }

    fn get_data_from_bigger(&self, row: Node, column: Node) -> Data {
        RestrictedDataMatrixSym::get_data_from_bigger(self, row, column)
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

pub struct ZeroIndexing;
pub struct OneIndexing;

#[derive(Debug, Clone)]
pub struct EdgeDataAdjList<Data, Indexing> {
    pub data: Vec<Data>,
    pub dimension: usize,
    indexing: PhantomData<Indexing>,
}

impl<Data, Indexing> EdgeDataAdjList<Data, Indexing> {
    pub fn new_zero_indexed(
        data: Vec<Data>,
        dimension: usize,
    ) -> EdgeDataAdjList<Data, ZeroIndexing> {
        debug_assert_eq!(data.len(), dimension * dimension);
        EdgeDataAdjList {
            data,
            dimension,
            indexing: PhantomData,
        }
    }

    pub fn new_one_indexed(
        data: Vec<Data>,
        dimension: usize,
    ) -> EdgeDataAdjList<Data, OneIndexing> {
        debug_assert_eq!(data.len(), dimension * dimension);
        EdgeDataAdjList {
            data,
            dimension,
            indexing: PhantomData,
        }
    }
}

impl<Data: Copy> EdgeDataAdjList<Data, ZeroIndexing> {
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

    /// Get the adjacency list for a given 'from' node.
    #[inline(always)]
    pub fn get_adjacency_list(&self, from: Node) -> &[Data] {
        let start_index = from.0 * self.dimension;
        &self.data[start_index..start_index + self.dimension]
    }
}

impl<Data: Copy> EdgeDataAdjList<Data, OneIndexing> {
    /// Access the data in a way that allows for faster sequential access when iterating over 'to'
    /// nodes.
    #[inline(always)]
    pub fn get_data_to_seq(&self, from: Node, to: Node) -> Data {
        let index = (from.0 - 1) * self.dimension + (to.0 - 1);
        self.data[index]
    }

    /// Access the data in a way that allows for faster sequential access when iterating over 'from'
    /// nodes.
    #[inline(always)]
    pub fn get_data_from_seq(&self, from: Node, to: Node) -> Data {
        self.get_data_to_seq(to, from)
    }

    /// Get the adjacency list for a given 'from' node.
    #[inline(always)]
    pub fn get_adjacency_list(&self, from: Node) -> &[Data] {
        let start_index = (from.0 - 1) * self.dimension;
        &self.data[start_index..start_index + self.dimension]
    }
}
