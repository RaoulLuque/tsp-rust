use crate::instance::{edge::data::EdgeDataMatrix, node::Node};

/// A row-major lower-triangular matrix to store arbitrary symmetric edge data.
///
/// The underlying data is guaranteed to have length dimension * (dimension + 1) / 2 where dimension
/// is the number of nodes.
#[derive(Debug, Clone)]
pub struct EdgeDataMatrixSym<Data> {
    data: Vec<Data>,
    dimension: usize,
}

impl<Data> EdgeDataMatrixSym<Data> {
    /// Create a new EdgeDataMatrixSym from raw data and dimension.
    ///
    /// Panics if the length of data does not equal dimension * dimension.
    pub fn new(data: Vec<Data>, dimension: usize) -> Self {
        assert_eq!(data.len(), dimension * (dimension + 1) / 2);
        Self { data, dimension }
    }

    /// Returns a reference to the underlying data.
    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    /// Set the data at (from, to).
    #[inline(always)]
    pub fn set_data(&mut self, from: Node, to: Node, data: Data) {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index] = data;
    }

    /// Set the data at (from, to), assuming 'from' is bigger than or equal to 'to'.
    ///
    /// May set the data for a wrong entry, if 'from' is smaller than 'to'.
    #[inline(always)]
    pub fn set_data_from_bigger(&mut self, from: Node, to: Node, data: Data) {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index] = data;
    }

    /// Set the data at (from, to), assuming 'to' is bigger than or equal to 'from'.
    ///
    /// May set the data for a wrong entry, if 'to' is smaller than 'from'.
    #[inline(always)]
    pub fn set_data_to_bigger(&mut self, from: Node, to: Node, data: Data) {
        self.set_data_from_bigger(to, from, data);
    }

    /// Creates a restricted view of the first n nodes of this EdgeDataMatrixSym.
    ///
    /// Panics if n > dimension.
    pub fn restrict_to_first_n<'a>(&'a self, n: usize) -> EDMSymViewRestricted<'a, Data> {
        EDMSymViewRestricted {
            data: &self.data[0..(n * (n + 1)) / 2],
            dimension: n,
        }
    }

    /// Create a new EdgeDataMatrixSym from a distance function.
    pub fn new_from_distance_function(
        dimension: usize,
        distance_function: impl Fn(Node, Node) -> Data,
    ) -> Self {
        let data: Vec<_> = (0..dimension)
            .flat_map(|row| (0..=row).map(move |column| (Node(row), Node(column))))
            .map(|(from, to)| distance_function(from, to))
            .collect();

        EdgeDataMatrixSym::new(data, dimension)
    }
}

impl<Data: Copy> EdgeDataMatrixSym<Data> {
    /// Access the data at (from, to).
    ///
    /// It might be faster to use `get_data_from_bigger` or `get_data_to_bigger` if you know
    /// which of 'from' or 'to' is bigger.
    #[inline(always)]
    pub fn get_data(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry(from.0, to.0);
        self.data[index]
    }

    /// Access the data at (from, to), assuming 'from' is bigger than or equal to 'to'.
    ///
    /// May return wrong data, if 'from' is smaller than 'to'.
    #[inline(always)]
    pub fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    /// Access the data at (from, to), assuming 'to' is bigger than or equal to 'from'.
    ///
    /// May return wrong data, if 'to' is smaller than 'from'.
    #[inline(always)]
    pub fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_row_bigger(to.0, from.0);
        self.data[index]
    }

    /// Convert to a non-symmetric [crate::instance::edge::data::EdgeDataMatrix] by duplicating the
    /// data.
    pub fn to_edge_data_matrix(&self) -> EdgeDataMatrix<Data> {
        let dimension = self.dimension;
        let mut data = vec![self.data[0].clone(); dimension * dimension];
        for row in 0..dimension {
            for column in 0..row {
                let value = self.get_data_from_bigger(Node(row), Node(column));
                data[row * self.dimension + column] = value.clone();
                data[column * self.dimension + row] = value;
            }
        }
        EdgeDataMatrix::new(data, self.dimension)
    }
}

impl<Data: Clone> EdgeDataMatrixSym<Data> {
    /// Create a new EdgeDataMatrixSym from dimension, filling all entries with the given value.
    pub fn new_from_dimension_with_value(dimension: usize, value: Data) -> Self {
        let size = (dimension * (dimension + 1)) / 2;
        EdgeDataMatrixSym::new(vec![value; size], dimension)
    }
}

/// A restricted view of an EdgeDataMatrixSym, only allowing access to the first n nodes.
#[derive(Debug, Clone)]
pub struct EDMSymViewRestricted<'a, Data> {
    data: &'a [Data],
    dimension: usize,
}

impl<'a, Data: Copy> EDMSymViewRestricted<'a, Data> {
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

    /// Access the data at (from, to), assuming 'from' is bigger than or equal to 'to'.
    ///
    /// May return wrong data, if 'from' is smaller than 'to'.
    #[inline(always)]
    pub fn get_data_from_bigger(&self, from: Node, to: Node) -> Data {
        let index = get_lower_triangle_matrix_entry_row_bigger(from.0, to.0);
        self.data[index]
    }

    /// Access the data at (from, to), assuming 'to' is bigger than or equal to 'from'.
    ///
    /// May return wrong data, if 'to' is smaller than 'from'.
    #[inline(always)]
    pub fn get_data_to_bigger(&self, from: Node, to: Node) -> Data {
        self.get_data_from_bigger(to, from)
    }
}

#[inline(always)]
/// Computes the index of entry (row, column) in a vec-flattened lower-(left-)triangular matrix.
pub fn get_lower_triangle_matrix_entry(row: usize, column: usize) -> usize {
    if row > column {
        get_lower_triangle_matrix_entry_row_bigger(row, column)
    } else {
        get_lower_triangle_matrix_entry_row_bigger(column, row)
    }
}

#[inline(always)]
/// Computes the index of entry (row, column) in a vec-flattened lower-(left-)triangular matrix,
/// assuming row >= column.
///
/// For a function that assumes column >= row, just switch the parameters to this function.
pub fn get_lower_triangle_matrix_entry_row_bigger(row: usize, column: usize) -> usize {
    debug_assert!(
        row >= column,
        "get_lower_triangle_matrix_entry_row_bigger called with row < column: row {}, column {}",
        row,
        column
    );
    (row * (row + 1)) / 2 + column
}
