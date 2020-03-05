use std::ops::Deref;

mod ops;

/// A generic matrix struct
///
/// keeps num of rows and cols and saves data in a 1 dim vector
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Allocates space for matrix with T's default value
    ///
    /// # Arguments
    ///
    /// * `rows` - num of rows
    /// * `cols` - num of cols
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Clone + Default + Copy,
    {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); cols * rows],
        }
    }

    /// Creates a matrix instance from 2 dim vectors
    ///
    /// # Arguments
    ///
    /// * `data` - two dimensional array of type T with data
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// If data doesn't contain a rectangular shape
    pub fn from(data: Vec<Vec<T>>) -> Matrix<T>
    where
        T: Clone,
    {
        assert!(data.windows(2).all(|w| w[0].len() == w[1].len()));

        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data.iter().flat_map(|s| s.clone()).collect(),
        }
    }

    /// Get number of cols
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get number of rows
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get a given coordinate
    ///
    /// Returns an option type
    ///
    /// # Arguments
    ///
    /// * `row` - row index
    /// * `col` - column index
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        match (row, col) {
            (x, y) if x < self.rows && y < self.cols => Some(&self.data[col + row * self.cols]),
            _ => None,
        }
    }

    /// Get an entire row
    ///
    /// Returns iterator of references
    /// if value needs to be saved, .unwrap().cloned() is needed
    ///
    /// # Arguments
    ///
    /// * `row` - row index
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        match row {
            x if x < self.rows => Some((0..self.cols).map(move |col| self.get(row, col).unwrap())),
            _ => None,
        }
    }

    /// Get an entire column
    ///
    /// Returns iterator of references
    /// if value needs to be saved, .unwrap().cloned() is needed
    ///
    /// # Arguments
    ///
    /// * `col` - column index
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        match col {
            x if x < self.cols => Some((0..self.rows).map(move |row| self.get(row, col).unwrap())),
            _ => None,
        }
    }

    /// Creates instance of transposed matrix
    ///
    /// # Example
    ///
    /// ```
    ///
    /// ```
    pub fn transpose(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for row in 0..self.rows {
                    for val in self.get_row(row).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod tests;
