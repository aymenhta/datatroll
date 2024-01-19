use std::{error::Error, fs};

#[derive(Debug)]
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
        if file_path.split(".").last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let content = fs::read_to_string(file_path)?;
        content.lines().for_each(|line| {
            let row: Vec<CellType> = line
                .split(",")
                .map(|s| s.trim())
                .map(|s| parse_string(s))
                .collect();
            self.data.push(row);
        });

        Ok(())
    }

    pub fn mean(&self, column: &str) -> f64 {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut sum = 0 as f64;

        for i in 1..self.data.len() {
            let val = match self.data[i][index] {
                CellType::IntCell(x) => x as f64,
                CellType::FloatCell(f) => f,
                _ => panic!("not supported"), // todo: see if we should propagate the error or not
            };

            sum = sum + val
        }

        sum / ((self.data.len() - 1) as f64)
    }

    pub fn max_int64(&self, column: &str) -> Result<i64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut max = 0 as i64;

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
        let mut max = 0 as f64;

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
        let mut min = 0 as i64;

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
        let mut min = 0 as f64;

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

    return CellType::StringCell(s.to_string());
}

#[cfg(test)]
mod tests;
