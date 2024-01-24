use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
};

/// Represents different types of data that can be stored in a cell.
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Null,
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
}

/// Represents a 2D array of cells, forming a sheet of data.
#[derive(Debug, Default)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    /// new_sheet initialize a Sheet
    pub fn new_sheet() -> Self {
        Self {
            data: Vec::<Vec<Cell>>::new(),
        }
    }

    /// Loads data from a CSV file into the Sheet's data structure.
    ///
    /// This function reads the content of a CSV file specified by `file_path` and populates
    /// the Sheet's data structure accordingly. The file must have a ".csv" extension, and
    /// its content should be in CSV (Comma-Separated Values) format.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the CSV file to load.
    ///
    /// # Errors
    ///
    /// Returns a `Result` indicating success or an error if the file cannot be opened,
    /// read, or if the file format is unsupported.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sheet = Sheet::default();
    ///
    /// if let Err(err) = sheet.load_data("input.csv") {
    ///     eprintln!("Error loading data: {}", err);
    /// } else {
    ///     println!("Data loaded successfully from input.csv");
    /// }
    /// ```
    pub fn load_data(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // check for ext
        if file_path.split('.').last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut content = String::new();

        reader.read_to_string(&mut content)?;

        content.lines().for_each(|line| {
            let row: Vec<Cell> = line.split(',').map(|s| s.trim()).map(parse_token).collect();
            self.data.push(row);
        });

        // if some column values are absent from a row, then fill it with a default Cell::Null
        let col_len = self.data[0].len();
        for i in 1..self.data.len() {
            let row_len = self.data[i].len();
            if row_len < col_len {
                for _ in 0..col_len - row_len {
                    self.data[i].push(Cell::Null);
                }
            }
        }

        Ok(())
    }

    /// Exports the content of a Sheet to a CSV file.
    ///
    /// The function writes the content of the Sheet into a CSV file specified by `file_path`.
    /// If the file already exists, it truncates the file and overwrites its content.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the CSV file.
    ///
    /// # Examples
    ///
    /// ```
    /// let cell_string = Cell::String(String::from("Hello, Rust!"));
    /// let cell_int = Cell::Int(42);
    ///
    /// let row1 = vec![cell_string, Cell::Bool(true), cell_int];
    /// let row2 = vec![Cell::Null, Cell::Float(3.14), Cell::String(String::from("World"))];
    ///
    /// let sheet = Sheet { data: vec![row1, row2] };
    ///
    /// if let Err(err) = sheet.export("output.csv") {
    ///     eprintln!("Error exporting data: {}", err);
    /// } else {
    ///     println!("Data exported successfully to output.csv");
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Result` indicating success or failure.
    ///
    pub fn export(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // check for ext
        if file_path.split('.').last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

        let mut buf_writer = BufWriter::new(file);

        for row in &self.data {
            for cell in row {
                match cell {
                    Cell::Null => write!(buf_writer, ",")?,
                    Cell::String(s) => write!(buf_writer, "{},", s)?,
                    Cell::Bool(b) => write!(buf_writer, "{},", b)?,
                    Cell::Int(i) => write!(buf_writer, "{},", i)?,
                    Cell::Float(f) => write!(buf_writer, "{},", f)?,
                }
            }
            writeln!(buf_writer)?; // Move to the next line after each row
        }

        buf_writer.flush()?; // Ensure any remaining data is written to the file
        Ok(())
    }

    /// insert_row appends a row to the data sheet at the last position
    ///
    /// The function takes a comma seperated input string, trim the whitespace, parse it into a
    /// vector oc Cell and then push it to the sheet.
    ///
    /// # Arguments
    ///
    /// * `input` - input string to be inserted.
    ///
    /// # Errors
    ///
    /// Returns a `Result` indicating success or an error if the input is of unvalid format
    ///
    /// # Examples
    ///
    /// ```
    /// let row1 = vec![Cell::String("Hello, Rust!".to_string()), Cell::Bool(true), Cell::Int(42)];
    /// let sheet = Sheet { data: vec![row1] };
    ///
    /// sheet.insert_row(",3.14,World")?;
    ///
    /// assert_eq!(sheet[0], row1);
    /// assert_eq!(sheet[1], vec![Cell::Null, Cell::Float(3.14), Cell::String("World".to_string()]);
    /// ```
    pub fn insert_row(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let row: Vec<Cell> = input
            .split(',')
            .map(|s| s.trim())
            .map(parse_token)
            .collect();
        if row.len() != self.data[0].len() {
            return Err(Box::from("invalid input"));
        }

        self.data.push(row);
        Ok(())
    }

    pub fn fill_col(&mut self, column: &str, value: Cell) -> Result<(), Box<dyn Error>> {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        for i in 1..self.data.len() {
            let cell = self.data[i]
                .get_mut(col_index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, i));

            *cell = value.clone();
        }

        Ok(())
    }

    pub fn paginate(&self, page: i32, size: i32) -> Vec<Vec<Cell>> {
        if page < 1 && size > 50 {
            panic!("page should more than or equal 1, size should 50 per page at max")
        }
        let mut res: Vec<Vec<Cell>> = Default::default();
        let offset = ((page - 1) * size) + 1;

        for i in offset..(offset + size) {
            let row = self.data.get(i as usize).unwrap_or_else(|| {
                panic!(
                    "offset '{}' and amount '{}' are out of bounds",
                    offset, size
                )
            });
            res.push(row.clone())
        }
        res
    }

    /// find_first_row return the first row in which a column cell satisfies a predicate,
    /// if otherwise it returns None
    pub fn find_first_row<F>(&self, column: &str, predicate: F) -> Option<&Vec<Cell>>
    where
        F: FnOnce(&Cell) -> bool + Copy,
    {
        let col_index = self.get_col_index(column).expect("column doesn't exist");

        for i in 1..self.data.len() {
            let cell = self.data[i]
                .get(col_index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, i));
            if predicate(cell) {
                return Some(&self.data[i]);
            }
        }

        None
    }

    pub fn find_rows<F>(&self, column: &str, predicate: F) -> Vec<Vec<Cell>>
    where
        F: FnOnce(&Cell) -> bool + Copy,
    {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let mut res: Vec<Vec<Cell>> = Default::default();

        for i in 1..self.data.len() {
            let cell = self.data[i]
                .get(col_index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, i));
            if predicate(cell) {
                res.push(self.data[i].clone());
            }
        }

        res
    }

    /// drop_rows delete all rows in which they contains cells that satisfies a provided predicate
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

    /// mode get the most frequent item of a column
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

    /// build_frequency_table gets the frequency of each elements in a column
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

    /// max_int64 return the maximum value of a column of integer values.
    /// if encountered with any type other than **Cell:Int(i64)** it exist an error.
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

    /// max_float64 return the maximum value of a column of float and integer values.
    /// if encountered with any type other than **Cell:Float(f64)** or **Cell::Int(i64)** it exist an error.
    pub fn max_float64(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut max = 0_f64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Float(f) => *f,
                Cell::Int(i) => *i as f64,
                _ => {
                    return Err(Box::from(
                        "max_float64 should only works on float and int values",
                    ))
                }
            };

            if max < row_val {
                max = row_val;
            }
        }

        Ok(max)
    }

    /// min_int64 return the minimum value of a column of integer values.
    /// if encountered with any type other than **Cell:Int(i64)** it exist an error.
    pub fn min_int64(&self, column: &str) -> Result<i64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut min = 0_i64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Int(x) => *x,
                _ => return Err(Box::from("min_int64 should only works on int values")),
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

    /// min_float64 return the minimum value of a column of float and integer values.
    /// if encountered with any type other than **Cell:Float(f64)** or **Cell::Int(i64)** it exist an error
    pub fn min_float64(&self, column: &str) -> Result<f64, Box<dyn Error>> {
        let index = self.get_col_index(column).expect("column doesn't exist");
        let mut min = 0_f64;

        for i in 1..self.data.len() {
            let row_val = match self.data[i]
                .get(index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", index, i))
            {
                Cell::Float(f) => *f,
                Cell::Int(i) => *i as f64,
                _ => {
                    return Err(Box::from(
                        "min_float64 should only works on float and int values",
                    ))
                }
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

    /// describe prints general infos about the sheet to the standard output in a formatted manner
    pub fn describe(&self) {
        println!("[");
        for i in 0..5 {
            print!("\t(");
            self.data[i].iter().for_each(|cell| match cell {
                Cell::String(s) => print!("{s},"),
                Cell::Bool(b) => print!("{b},"),
                Cell::Int(x) => print!("{x},"),
                Cell::Float(f) => print!("{f},"),
                Cell::Null => print!(" ,"),
            });
            println!(")");
        }

        let col_len = self.data[0].len();
        for _ in 0..col_len * 10 {
            print!("-");
        }
        println!();

        let len = self.data.len();
        for i in len - 5..len {
            print!("\t(");
            self.data[i].iter().for_each(|cell| match cell {
                Cell::String(s) => print!("{s},"),
                Cell::Bool(b) => print!("{b},"),
                Cell::Int(x) => print!("{x},"),
                Cell::Float(f) => print!("{f},"),
                Cell::Null => print!(" ,"),
            });
            println!(")");
        }
        println!("]");

        println!(
            "
            number of rows: {len}
            number of columns: {col_len}"
        )
    }

    /// pretty_print prints the sheet to the standard output in a formatted manner
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

    /// get_col_index returns the index of a given column, and None otherwise
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

/// parse_string takes a token string, parses it and returns in the form of a Cell
fn parse_token(token: &str) -> Cell {
    if token == "true" {
        return Cell::Bool(true);
    }

    if token == "false" {
        return Cell::Bool(false);
    }

    if let Ok(i) = token.parse::<i64>() {
        return Cell::Int(i);
    }

    if let Ok(f) = token.parse::<f64>() {
        return Cell::Float(f);
    }

    if token.is_empty() {
        return Cell::Null;
    }

    Cell::String(token.to_string())
}

#[cfg(test)]
mod tests;
