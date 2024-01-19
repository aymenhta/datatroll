use std::{error::Error, fs};

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    StringCell(String),
    BooleanCell(bool),
    IntCell(i64),
    FloatCell(f64),
}

#[derive(Debug, Default)]
pub struct Sheet {
    pub data: Vec<Vec<CellType>>,
}

impl Sheet {
    pub fn new_sheet() -> Self {
        Self {
            data: Vec::<Vec<CellType>>::new(),
        }
    }

    pub fn load_data(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // check for ext
        if file_path.split('.').last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let content = fs::read_to_string(file_path)?;
        content.lines().for_each(|line| {
            let row: Vec<CellType> = line
                .split(',')
                .map(|s| s.trim())
                .map(parse_string)
                .collect();
            self.data.push(row);
        });

        Ok(())
    }

    pub fn mean(&self, column: &str) -> f64 {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut sum = 0_f64;

        for i in 1..self.data.len() {
            let val = match self.data[i][index] {
                CellType::IntCell(x) => x as f64,
                CellType::FloatCell(f) => f,
                _ => panic!("not supported"), // todo: see if we should propagate the error or not
            };

            sum += val
        }

        sum / ((self.data.len() - 1) as f64)
    }

    /// median calculates the value in the middle of the provided column
    pub fn median(&self, column: &str) -> CellType {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let row_index = ((self.data.len() - 1) + 1) / 2;

        self.data[row_index][col_index].clone()
    }

    // mode get the most frequent item of a column
    // todo: also support Bimodal, Trimodal & Multimodal
    pub fn mode(&self, column: &str) -> (CellType, i32) {
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

    fn build_frequency_table(&self, col_index: usize) -> Vec<(CellType, i32)> {
        let mut frequency_table: Vec<(CellType, i32)> = Vec::new();

        for i in 1..self.data.len() {
            let cell = &self.data[i][col_index];
            // for item in &mut frequency_table {
            //     if item.0 == *cell {
            //         item.1 = item.1 + 1;
            //         continue;
            //     } else {
            //         frequency_table.push((cell.clone(), 1))
            //     }
            // }
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
            let row_val = match self.data[i][index] {
                CellType::IntCell(x) => x,
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
            let row_val = match self.data[i][index] {
                CellType::FloatCell(x) => x,
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
            let row_val = match self.data[i][index] {
                CellType::IntCell(x) => x,
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
            let row_val = match self.data[i][index] {
                CellType::FloatCell(x) => x,
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

    fn get_col_index(&self, column: &str) -> Option<usize> {
        for i in 0..self.data[0].len() {
            let colname = match &self.data[0][i] {
                CellType::StringCell(s) => s,
                _ => panic!("not supported type for head"),
            };

            if colname == column {
                return Some(i);
            }
        }

        None
    }
}

fn parse_string(s: &str) -> CellType {
    if s == "true" {
        return CellType::BooleanCell(true);
    }

    if s == "false" {
        return CellType::BooleanCell(false);
    }

    if let Ok(i) = s.parse::<i64>() {
        return CellType::IntCell(i);
    }

    if let Ok(f) = s.parse::<f64>() {
        return CellType::FloatCell(f);
    }

    CellType::StringCell(s.to_string())
}

#[cfg(test)]
mod tests;
