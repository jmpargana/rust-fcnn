pub struct Matrix<T> {
   rows: usize,
   cols: usize,
   data: Vec<Vec<T>>,
}


impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T> 
        where T: Clone + Default + Copy {
        Matrix {
            rows,
            cols,
            data: vec![vec![T::default(); cols]; rows]
        }
    }

    pub fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        match (row, col) {
            (x, y) if x < self.rows && y < self.cols => Some(&self.data[row][col]),
            _ => None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<&Vec<T>> {
        match row {
            x if x < self.rows => Some(&self.data[x]),
            _ => None
        }
    }

    pub fn get_col(&self, col: usize) -> Option<&Vec<T>> 
        where T: Copy {
        match col {
            x if x < self.cols => {
                let column = (0..self.rows)
                    .map(move |row| *self.get(row, col).unwrap()).collect();
                Some(column)
            }
            _ => None
        }
    }
}
