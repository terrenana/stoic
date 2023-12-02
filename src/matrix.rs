use na::DMatrix;
use num::rational::Ratio;
use num::{One, Signed, Zero};

pub struct GaussianElimination {
    matrix_a: DMatrix<Ratio<isize>>, // A n*m matrix.
    n: usize,
    m: usize,
}

impl GaussianElimination {
    pub fn new(matrix_a: DMatrix<Ratio<isize>>) -> Self {
        // Create a GaussianElimination Solution.
        let (n, m) = matrix_a.shape();
        Self { matrix_a, n, m }
    }

    pub fn solve(mut self) -> Vec<Vec<Ratio<isize>>> {
        // The Gaussian-Jordan Algorithm
        let mut var_table = Vec::<usize>::new();
        for i in 0..self.n {
            let mostleft_row = match self.get_leftmost_row(i) {
                Some(s) => s,
                None => continue,
            };
            let j = match self.get_pivot(mostleft_row) {
                Some(s) => {
                    var_table.push(s);
                    s
                }
                None => continue, // if most left row has no pivot, just continue.
            };
            let max_row = self.get_max_abs_row(i, j);
            if self.matrix_a[(max_row, j)] != Ratio::<isize>::zero() {
                self.matrix_a.swap_rows(i, max_row); // swap row i and maxi in matrix_a
                {
                    let tmp = &(self.matrix_a.row(i) / self.matrix_a[(i, j)]);
                    self.matrix_a.row_mut(i).copy_from(tmp);
                }
                for u in i + 1..self.n {
                    let v = self.matrix_a.row(i) * self.matrix_a[(u, j)];
                    for (k, item) in v.iter().enumerate().take(self.m) {
                        self.matrix_a[(u, k)] -= *item; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                    }
                }
            }
        } // REF
        for i in (0..self.n).rev() {
            let j = match self.get_pivot(i) {
                Some(s) => s,
                None => continue,
            };
            for u in (0..i).rev() {
                // j above i
                let v = self.matrix_a.row(i) * self.matrix_a[(u, j)];
                for (k, item) in v.iter().enumerate().take(self.m) {
                    self.matrix_a[(u, k)] -= *item; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                }
            }
        } // RREF
        let v = (0..self.n)
            .filter(|i| self.matrix_a.row(*i).iter().all(Zero::is_zero))
            .collect::<Vec<_>>();
        self = self.simplify(v); // eliminate the zero rows
        var_table = (0..self.m)
            .filter(|e| !var_table.contains(&e))
            .collect::<Vec<_>>(); // get free variables table
        var_table.iter().for_each(|x| {
            let tmp = self.matrix_a.column(*x) * -Ratio::<isize>::one();
            self.matrix_a.column_mut(*x).copy_from(&tmp)
        });
        let mut ans = var_table
            .iter()
            .map(|i| self.matrix_a.column(*i).iter().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let len = var_table.len();
        for (i, j) in var_table.into_iter().enumerate() {
            for (v, item) in ans.iter_mut().enumerate().take(len) {
                if i == v {
                    item.insert(j, Ratio::<isize>::one());
                } else {
                    item.insert(j, Ratio::<isize>::zero());
                }
            }
        }
        if ans.is_empty() {
            panic!("no solution")
        } else {
            ans
        }
    }

    fn simplify(mut self, list: Vec<usize>) -> Self {
        for i in list.into_iter().rev() {
            self.matrix_a = self.matrix_a.remove_row(i);
        }
        let (n, m) = self.matrix_a.shape();
        Self {
            matrix_a: self.matrix_a,
            n,
            m,
        }
    }

    fn get_pivot(&self, row: usize) -> Option<usize> {
        for column in 0..self.m {
            if self.matrix_a[(row, column)] != Ratio::<isize>::zero() {
                return Some(column);
            }
        }
        None
    }

    fn get_leftmost_row(&self, row: usize) -> Option<usize> {
        let mut lock = false;
        // Use `lock` to prevent calculation from `usize` Overflow
        let mut mostleft_row = row;
        let mut min_left: usize = match self.get_pivot(row) {
            Some(s) => s,
            None => {
                lock = true;
                0
            }
        };
        for i in row + 1..self.n {
            let current_pivot = match self.get_pivot(i) {
                Some(s) => s,
                None => continue,
            };
            if (current_pivot < min_left) | (lock) {
                mostleft_row = i;
                min_left = current_pivot;
                lock = false;
            }
        }
        if lock {
            None
        } else {
            Some(mostleft_row)
        }
    }

    fn get_max_abs_row(&self, row: usize, column: usize) -> usize {
        let mut maxi = row;
        for k in row + 1..self.n {
            if self.matrix_a[(k, column)].abs() > self.matrix_a[(maxi, column)].abs() {
                maxi = k;
            }
        }
        maxi
    }
}