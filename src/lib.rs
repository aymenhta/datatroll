use std::{error::Error, fs};

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Null,
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Default)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    pub fn new_sheet() -> Self {
        Self {
            data: Vec::<Vec<Cell>>::new(),
        }
    }

    // TODO: ALSO SUPPORT EXPORTING DATA TO CSV/JSON FILES
    pub fn load_data(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // check for ext
        if file_path.split('.').last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let content = fs::read_to_string(file_path)?;
        content.lines().for_each(|line| {
            let row: Vec<Cell> = line
                .split(',')
                .map(|s| s.trim())
                .map(parse_string)
                .collect();
            self.data.push(row);
        });

        Ok(())
    }

    pub fn insert_row(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let row: Vec<Cell> = input
            .split(',')
            .map(|s| s.trim())
            .map(parse_string)
            .collect();
        if row.len() != self.data[0].len() {
            return Err(Box::from("invalid input"));
        }

        self.data.push(row);
        Ok(())
    }

    pub fn drop_rows<F>(&mut self, column: &str, predicate: F)
    where
        F: FnOnce(&Cell) -> bool + Copy,
    {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        self.data.retain(|row| !predicate(&row[col_index]));
    }

    pub fn drop_col(&mut self, column: &str) {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        for i in 0..self.data.len() {
            self.data[i].remove(col_index);
        }
    }

    /// Mean is usually represented by x-bar or x̄.
    ///
    /// X̄ = (Sum of values ÷ Number of values in data set)
    pub fn mean(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut sum = 0_f64;

        for i in 1..self.data.len() {
            let val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Int(x) => *x as f64,
                Cell::Float(f) => *f,
                _ => return Err(Box::from("column value should be an i64 or a f64")),
            };

            sum += val
        }

        Ok(sum / ((self.data.len() - 1) as f64))
    }

    /// The formula to find the variance is given by:
    /// Var (X) = E[( X – μ)²] Where Var (X) is the variance
    /// E denotes the expected value
    /// X is the random variable and μ is the mean
    pub fn variance(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let mean = self.mean(column)?;

        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut total_sum = 0_f64;
        for i in 1..self.data.len() {
            let val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Int(x) => *x as f64,
                Cell::Float(f) => *f,
                _ => return Err(Box::from("column value should be an i64 or a f64")),
            };

            total_sum += (val - mean).powf(2.0)
        }

        Ok(total_sum / (self.data.len() - 1) as f64)
    }

    /// median calculates the value in the middle of the provided column
    pub fn median(&self, column: &str) -> &Cell {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let row_index = ((self.data.len() - 1) + 1) / 2;

        self.data[row_index]
            .get(col_index)
            .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, row_index))
    }

    // mode get the most frequent item of a column
    // TODO: also support Bimodal, Trimodal & Multimodal
    pub fn mode(&self, column: &str) -> (Cell, i32) {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let fq = self.build_frequency_table(col_index);
        let mut max = 0;
        let mut max_index = 0_usize;

        for (i, item) in fq.iter().enumerate() {
            if max < item.1 {
                max = item.1;
                max_index = i;
            }
        }

        fq[max_index].clone()
    }

    fn build_frequency_table(&self, col_index: usize) -> Vec<(Cell, i32)> {
        let mut frequency_table: Vec<(Cell, i32)> = Vec::new();

        for i in 1..self.data.len() {
            let cell = self.data[i]
                .get(col_index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, i));
            if frequency_table.is_empty() {
                frequency_table.push((cell.clone(), 1));
                continue;
            }

            let index = frequency_table.iter().position(|item| item.0 == *cell);
            if let Some(idx) = index {
                frequency_table[idx].1 += 1;
            } else if index.is_none() {
                frequency_table.push((cell.clone(), 1));
            }
        }

        frequency_table
    }

    pub fn max_int64(&self, column: &str) -> Result<i64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut max = 0_i64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Int(x) => *x,
                _ => return Err(Box::from("max_int64 should only works on int values")),
            };

            if max < row_val {
                max = row_val;
            }
        }

        Ok(max)
    }

    pub fn max_float64(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut max = 0_f64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Float(f) => *f,
                _ => return Err(Box::from("max_int64 should only works on int values")),
            };

            if max < row_val {
                max = row_val;
            }
        }

        Ok(max)
    }

    pub fn min_int64(&self, column: &str) -> Result<i64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut min = 0_i64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Int(x) => *x,
                _ => return Err(Box::from("max_int64 should only works on int values")),
            };

            if i == 1 {
                min = row_val;
                continue;
            }

            if min > row_val {
                min = row_val;
            }
        }

        Ok(min)
    }

    pub fn min_float64(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut min = 0_f64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Float(f) => *f,
                _ => return Err(Box::from("max_int64 should only works on int values")),
            };

            if i == 1 {
                min = row_val;
                continue;
            }

            if min > row_val {
                min = row_val;
            }
        }

        Ok(min)
    }

    pub fn pretty_print(&self) {
        println!("[");
        self.data.iter().for_each(|row| {
            print!("\t(");
            row.iter().for_each(|cell| match cell {
                Cell::String(s) => print!("{s},"),
                Cell::Bool(b) => print!("{b},"),
                Cell::Int(x) => print!("{x},"),
                Cell::Float(f) => print!("{f},"),
                Cell::Null => print!(" ,"),
            });
            println!(")");
        });
        println!("]");
    }

    fn get_col_index(&self, column: &str) -> Option<usize> {
        for i in 0..self.data[0].len() {
            if let Cell::String(colname) = &self.data[0][i] {
                if colname == column {
                    return Some(i);
                }
            };
        }

        None
    }
}

fn parse_string(s: &str) -> Cell {
    if s == "true" {
        return Cell::Bool(true);
    }

    if s == "false" {
        return Cell::Bool(false);
    }

    if let Ok(i) = s.parse::<i64>() {
        return Cell::Int(i);
    }

    if let Ok(f) = s.parse::<f64>() {
        return Cell::Float(f);
    }

    if s.is_empty() {
        return Cell::Null;
    }

    Cell::String(s.to_string())
}

#[cfg(test)]
mod tests;
