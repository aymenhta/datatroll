//! datatroll is a robust and user-friendly Rust library for efficiently loading, manipulating,
//! and exporting data stored in CSV files. Say goodbye to tedious hand-coding data parsing and
//! welcome a streamlined workflow for wrangling your data with ease.
//!
//! ## Features:
//! - **Versatile Data Loading:**
//!   - Read data from CSV files with configurable separators and headers.
//!   - Specify data types for each column, ensuring type safety and efficient processing.
//!   - Handle missing values with graceful error handling.
//! - **Intuitive Data Manipulation:**
//!     - Insert new rows with custom values into your data.
//!     - Drop unwanted rows or columns to focus on relevant data.
//!     - Leverage powerful aggregations to calculate:
//!         - Mean, max, min, and median of numeric columns.
//!         - Mode (most frequent value) of categorical columns.
//!         - Variance of numeric columns.
//!     - Apply custom transformations to specific columns using lambda functions.
//!     - Supports Pagination
//! - **Seamless Data Export:**
//!     - Write manipulated data back to a new CSV file, retaining original format or specifying your own.
//!     - Customize output with options like separator selection and header inclusion.
//!
//! # Example:
//! ```rust
//! use datatroll::{Cell, Sheet};
//!
//! fn main() {
//!     // Read data from a CSV file
//!     let data = "id ,title , director, release date, review
//!1, old, quintin, 2011, 3.5
//!2, her, quintin, 2013, 4.2
//!3, easy, scorces, 2005, 1.0
//!4, hey, nolan, 1997, 4.7
//!5, who, martin, 2017, 5.0";
//!     let mut sheet = Sheet::load_data_from_str(data);
//!
//!     // drop all the rows in which the review is less than 4.0
//!     sheet.drop_rows("review", |c| {
//!         if let Cell::Float(r) = c {
//!             return *r < 4.0;
//!         }
//!         false
//!     });
//!
//!     // calculate the variance of the review column
//!     let variance = sheet.variance("review").unwrap();
//!     println!("variance for review is: {variance}");
//!     
//!     // Write the transformed data to a new CSV file
//!     if let Err(err) = sheet.export("output.csv") {
//!         eprintln!("Error exporting data: {}", err);
//!     } else {
//!         println!("Data exported successfully to output.csv");
//!     }
//! }
//! ```

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
};

/// Represents different types of data that can be stored in a cell.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cell {
    Null,
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
}

/// Represents a 2D vector of cells, forming a sheet of data.
#[derive(Debug, Default)]
pub struct Sheet {
    /// 2D vector of cells
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    /// new_sheet initialize a Sheet
    fn new_sheet() -> Self {
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
    /// ```rust
    /// use datatroll::Sheet;
    ///
    /// if let Err(err) = Sheet::load_data("input.csv") {
    ///     eprintln!("Error loading data: {}", err);
    /// } else {
    ///     println!("Data loaded successfully from input.csv");
    /// }
    /// ```
    pub fn load_data(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut sheet = Self::new_sheet();
        // check for ext
        if file_path.split('.').last() != Some("csv") {
            return Err(Box::from(
                "the provided file path is invalid, or of unsupported format",
            ));
        }

        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut data = String::new();

        reader.read_to_string(&mut data)?;

        data.lines().for_each(|line| {
            let row: Vec<Cell> = line.split(',').map(|s| s.trim()).map(parse_token).collect();
            sheet.data.push(row);
        });

        // if some column values are absent from a row, then fill it with a default Cell::Null
        let col_len = sheet.data[0].len();
        for i in 1..sheet.data.len() {
            let row_len = sheet.data[i].len();
            if row_len < col_len {
                for _ in 0..col_len - row_len {
                    sheet.data[i].push(Cell::Null);
                }
            }
        }

        Ok(sheet)
    }

    pub fn load_data_from_str(data: &str) -> Self {
        let mut sheet = Self::new_sheet();

        data.lines().for_each(|line| {
            let row: Vec<Cell> = line.split(',').map(|s| s.trim()).map(parse_token).collect();
            sheet.data.push(row);
        });

        // if some column values are absent from a row, then fill it with a default Cell::Null
        let col_len = sheet.data[0].len();
        for i in 1..sheet.data.len() {
            let row_len = sheet.data[i].len();
            if row_len < col_len {
                for _ in 0..col_len - row_len {
                    sheet.data[i].push(Cell::Null);
                }
            }
        }

        sheet
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
    /// ```rust
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
    /// ```rust
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

    /// fill_col replace the value of a column in every row
    ///
    /// The function takes a column name and the value to be filled, and iterate through every row
    /// and effectively replace its old cell values with the new value
    ///
    /// # Arguments
    ///
    /// * `column` - the column to be mutated
    /// * `value` - the value which every row will be filled with
    ///
    /// # Errors
    ///
    /// Returns a `Result` indicating success or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// let row1 = vec![Cell::String("greeting".to_string()), Cell::String("is_good".to_string()), Cell::String("count".to_string())];
    /// let row2 = vec![Cell::String("Hello, Rust!".to_string()), Cell::Bool(false), Cell::Int(42)];
    /// let row3 = vec![Cell::String("Hello, World!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let sheet = Sheet { data: vec![row1, row2, row3] };
    ///
    /// sheet.fill_col("greeting", Cell::Null)?;
    ///
    /// assert_eq!(sheet[1][0], Cell::Null);
    /// assert_eq!(sheet[1][0], Cell::Null);
    /// ```
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

    /// paginate takes part of a sheet with a fixed size and return it
    ///
    /// The function takes a page number and a page size, and slice the sheet and returns it as a page
    /// of fixed size
    ///
    /// # Arguments
    ///
    /// * `page` - the number of the page
    /// * `size` - number of rows for every page
    ///
    /// # Errors
    ///
    /// Returns a `Result` indicating success or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// let row1 = vec![Cell::String("greeting".to_string()), Cell::String("is_good".to_string()), Cell::String("count".to_string())];
    /// let row2 = vec![Cell::String("Hello, Rust!".to_string()), Cell::Bool(false), Cell::Int(42)];
    /// let row3 = vec![Cell::String("Hello, World!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let row4 = vec![Cell::String("Hello, Dzair!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let row5 = vec![Cell::String("Hello, Africa!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let row6 = vec![Cell::String("Hello, Algeria!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let row7 = vec![Cell::String("Hello, Friday!".to_string()), Cell::Bool(true), Cell::Int(145)];
    /// let sheet = Sheet { data: vec![row1, row2, row3, row4, row5, row6, row7] };
    ///
    /// let page = sheet.paginate(1, 2)?;
    ///
    /// assert_eq!(page[0][0], Cell::String("Hello, Rust!".to_string()));
    /// assert_eq!(page[1][0], Cell::String("Hello, World!".to_string()));
    /// ```
    pub fn paginate(&self, page: usize, size: usize) -> Result<Vec<Vec<Cell>>, Box<dyn Error>> {
        if page < 1 || size > 50 {
            return Err(Box::from(
                "page should more than or equal 1, size should 50 per page at max",
            ));
        }
        if self.data.len() < size {
            return Err(Box::from("page unavailabe"));
        }

        let mut res: Vec<Vec<Cell>> = Default::default();
        let offset = ((page - 1) * size) + 1;

        for i in offset..(offset + size) {
            let row = self.data.get(i).unwrap_or_else(|| {
                panic!(
                    "offset '{}' and amount '{}' are out of bounds",
                    offset, size
                )
            });
            res.push(row.clone())
        }

        Ok(res)
    }

    /// Finds the first row in the table that matches a predicate applied to a specific column.
    ///
    /// # Panics
    ///
    /// Panics if the specified column doesn't exist or is absent for a row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let first_matching_rows = sheet.find_rows("Age", |cell| cell.as_int() >= 30);
    /// ```
    ///
    /// # Generics
    ///
    /// The `predicate` argument is a generic function that allows for flexible filtering criteria.
    /// It accepts a reference to a `Cell` and returns a boolean indicating whether the row matches.
    ///
    /// # Returns
    ///
    /// An `Option<&Vec<Cell>>`:
    /// - `Some(&row)` if a matching row is found, where `row` is a reference to the first matching row.
    /// - `None` if no matching row is found.
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

    /// Finds rows in the table that match a predicate applied to a specific column.
    ///
    /// # Panics
    ///
    /// Panics if the specified column doesn't exist or is absent for a row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let matching_rows = sheet.filter("Age", |cell| cell.as_int() >= 30);
    /// ```
    ///
    /// # Generics
    ///
    /// The `predicate` argument is a generic function that allows for flexible filtering criteria.
    /// It accepts a reference to a `Cell` and returns a boolean indicating whether the row matches.
    ///
    /// # Returns
    ///
    /// A vector of vectors, where each inner vector represents a row that matches the predicate.
    pub fn filter<F>(&self, column: &str, predicate: F) -> Vec<Vec<Cell>>
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

    /// The map function applies a given transformation to each column value of rows.
    ///
    /// # Errors
    ///
    /// Returns a `Result` indicating success or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use datatroll::{Sheet, Cell};
    ///
    ///let data = "id ,title , director, release date, review
    ///1, old, quintin, 2011, 3.5
    ///2, her, quintin, 2013, 4.2
    ///3, easy, scorces, 2005, 1.0
    ///4, hey, nolan, 1997, 4.7
    ///5, who, martin, 2017, 5.0";
    ///
    /// let mut sheet = Sheet::load_data_from_str(data);
    ///
    /// let result = sheet.map("title", |c| match c {
    ///     Cell::String(s) => Cell::String(s.to_uppercase()),
    ///     _ => return c,
    /// });
    /// 
    /// assert!(result.is_ok());
    /// ```
    pub fn map<F>(&mut self, column: &str, transform: F) -> Result<(), String>
    where
        F: Fn(Cell) -> Cell,
    {
        match self.get_col_index(column) {
            Some(i) => {
                self.data
                    .iter_mut()
                    .for_each(|row| row[i] = transform(row[i].clone()));
                Ok(())
            }
            None => Err(format!("could not find column '{column}'")),
        }
    }

    /// Removes rows from the table based on a predicate applied to a specific column.
    ///
    /// # Panics
    ///
    /// Panics if the specified column doesn't exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// sheet.drop_rows("Age", |cell| cell.as_int() >= 30); // Removes rows where age is 30 or older
    /// ```
    ///
    /// # Generics
    ///
    /// The `predicate` argument is a generic function that allows for flexible filtering criteria.
    /// It accepts a reference to a `Cell` and returns a boolean indicating whether to keep the row.
    pub fn drop_rows<F>(&mut self, column: &str, predicate: F)
    where
        F: FnOnce(&Cell) -> bool + Copy,
    {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        self.data.retain(|row| !predicate(&row[col_index]));
    }

    /// Removes a specified column from the table and returns the number of rows affected.
    ///
    /// # Panics
    ///
    /// Panics if the specified column doesn't exist.
    ///
    /// # Returns
    ///
    /// The number of rows that were modified by removing the column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let rows_affected = sheet.drop_col("id") // Removes the "id" column and returns 5
    /// ```
    pub fn drop_col(&mut self, column: &str) -> i32 {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let mut rows_affected = 0;
        for i in 0..self.data.len() {
            self.data[i].remove(col_index);
            rows_affected += 1;
        }

        rows_affected
    }

    /// Calculates the mean (average) of a specified column.
    ///
    /// The mean is the sum of all values in a data set divided by the number of values.
    ///
    /// # Formula
    ///
    /// X̄ = (ΣX) / N
    ///
    /// Where:
    /// - X̄ is the mean
    /// - ΣX is the sum of all values in the column
    /// - N is the number of values in the column
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-numeric values (i.e., not `i64` or `f64`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let re_mean = sheet.mean("release year")?; // Returns the mean of the "Age" column
    /// ```
    ///
    /// # Returns
    ///
    /// The mean of the specified column as an `f64`, or an error if one occurs.
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

    /// Calculates the variance of a specified column.
    ///
    /// Variance measures how far a set of numbers are spread out from their average value.
    /// It is calculated as the average of the squared differences from the mean.
    ///
    /// # Formula
    ///
    /// Var(X) = E[(X - μ)²]
    ///
    /// Where:
    /// - Var(X) is the variance
    /// - E denotes the expected value (average)
    /// - X is the random variable (the values in the column)
    /// - μ is the mean of X
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-numeric values (i.e., not `i64` or `f64`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let re_variance = sheet.variance("release year")?; // Returns the variance of the "release year" column
    /// ```
    ///
    /// # Returns
    ///
    /// The variance of the specified column as an `f64`, or an error if one occurs.
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

    /// Calculates the median value of a specified column.
    ///
    /// The median is the value that separates the higher half of a data set from the lower half.
    /// In this case, it's the value that falls in the middle of the column when the data is sorted.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column is absent for the middle row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    /// let median_id = sheet.median("id")?; // Returns a &Int(3)
    /// ```
    /// # Returns
    ///
    /// A reference to the `Cell` containing the median value of the specified column.
    pub fn median(&self, column: &str) -> &Cell {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let row_index = ((self.data.len() - 1) + 1) / 2;

        self.data[row_index]
            .get(col_index)
            .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, row_index))
    }

    /// mode get the most frequent items of a column
    ///
    /// The function gets a vector of the most frequent items in a column, alongside their number of
    /// occurences.
    ///
    /// # Arguments
    ///
    /// * `columnn` - the name of the column
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut sheet = Sheet::new_sheet();
    /// sheet.load_data("test_data.csv").unwrap();
    ///
    /// let multimodal = sheet.mode("director");
    /// println!("mode: {:?}", multimodal) // mode: [(String("quintin"), 2), (String("martin"), 2)]
    ///```
    pub fn mode(&self, column: &str) -> Vec<(Cell, i32)> {
        let col_index = self.get_col_index(column).expect("column doesn't exist");
        let fq = self.build_frequency_table(col_index);
        let mut max = 0;
        let mut multi_mode: Vec<(Cell, i32)> = Vec::new();

        for item in fq.iter() {
            if max <= item.1 {
                max = item.1;
                multi_mode.push(item.clone());
            }
        }

        multi_mode
    }

    /// Builds a frequency table for a specified column, counting the occurrences of each unique value.
    ///
    /// # Panics
    ///
    /// Panics if the specified column doesn't exist or is absent for a row.
    ///
    /// # Returns
    ///
    /// A vector of tuples `(Cell, i32)`, where:
    /// - `Cell` is the unique value from the column.
    /// - `i32` is the frequency (count) of that value in the column.
    fn build_frequency_table(&self, col_index: usize) -> Vec<(Cell, i32)> {
        let mut fq: Vec<(Cell, i32)> = Vec::new();

        for i in 1..self.data.len() {
            let cell = self.data[i]
                .get(col_index)
                .unwrap_or_else(|| panic!("column '{}' is absent for row '{}'", col_index, i));
            if fq.is_empty() {
                fq.push((cell.clone(), 1));
                continue;
            }

            let index = fq.iter().position(|item| item.0 == *cell);
            if let Some(idx) = index {
                fq[idx].1 += 1;
            } else if index.is_none() {
                fq.push((cell.clone(), 1));
            }
        }

        fq
    }

    /// Finds the maximum value of a specified column, specifically for `i64` values.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-integer values (i.e., not `i64`).
    ///
    /// # Returns
    ///
    /// The maximum `i64` value in the specified column, or an error if one occurs.
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

    /// Finds the maximum value of a specified column, working with both `f64` and `i64` values.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-numeric values (i.e., not `f64` or `i64`).
    ///
    /// # Returns
    ///
    /// The maximum value in the specified column, either an `f64` or an `i64` cast to `f64`, or an error if one occurs.
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

    /// Finds the minimum value of a specified column, specifically for `i64` values.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-integer values (i.e., not `i64`).
    ///
    /// # Returns
    ///
    /// The minimum `i64` value in the specified column, or an error if one occurs.
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

    /// Finds the minimum value of a specified column, working with both `f64` and `i64` values.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The specified column doesn't exist.
    /// - The specified column contains non-numeric values (i.e., not `f64` or `i64`).
    ///
    /// # Returns
    ///
    /// The minimum value in the specified column, either an `f64` or an `i64` cast to `f64`, or an error if one occurs.
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

    /// Prints general information about the sheet to the standard output in a formatted manner.
    ///
    /// This includes:
    ///
    /// - The first 5 rows of the sheet.
    /// - A separator line.
    /// - The last 5 rows of the sheet.
    /// - The total number of rows and columns
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
                Cell::Null => print!("NULL,"),
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

    /// Prints the entire sheet to the standard output in a formatted manner.
    ///
    /// Each row is enclosed in parentheses and separated by commas, providing a visual representation of the sheet's structure and content.
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

/// Parses a string token into the appropriate Cell type.
///
/// # Behavior
///
/// - Returns `Cell::Bool(true)` for the token "true".
/// - Returns `Cell::Bool(false)` for the token "false".
/// - Returns `Cell::Int(i64)` if the token can be parsed as an integer.
/// - Returns `Cell::Float(f64)` if the token can be parsed as a floating-point number.
/// - Returns `Cell::Null` if the token is empty.
/// - Returns `Cell::String(token.to_string())` for any other string value.
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
