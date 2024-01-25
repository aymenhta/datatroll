## datatroll: Powerfully Wrangle Your CSV Data in Rust
datatroll is a robust and user-friendly Rust library for efficiently loading, manipulating, and exporting data stored in CSV files. Say goodbye to tedious hand-coding data parsing and welcome a streamlined workflow for wrangling your data with ease.

### Features:
- **Versatile Data Loading:**
  - Read data from CSV files with configurable separators and headers.
  - Specify data types for each column, ensuring type safety and efficient processing.
  - Handle missing values with graceful error handling.
- **Intuitive Data Manipulation:**
    - Insert new rows with custom values into your data.
    - Drop unwanted rows or columns to focus on relevant data.
    - Leverage powerful aggregations to calculate:
        - Mean, max, min, and median of numeric columns.
        - Mode (most frequent value) of categorical columns.
        - Variance of numeric columns.
    - Apply custom transformations to specific columns using lambda functions.
    - Supports Pagination
- **Seamless Data Export:**
    - Write manipulated data back to a new CSV file, retaining original format or specifying your own.
    - Customize output with options like separator selection and header inclusion.
### Benefits:
- **Save Time and Effort:** Focus on analyzing data, not wrangling it.
- **Minimize Errors:** Type-safe data handling and clear error messages improve code reliability.
- **Boost Productivity:** Get more done with faster data loading, manipulation, and export.
- **Write Concise Code:** Enjoy an intuitive API for concise and expressive data wrangling tasks.
### Getting Started:
Add rust-csv-wrangler to your project with Cargo:
```toml
[dependencies]
datatroll = "0.1.0"
```
Import the library and start wrangling your data:

```rust
use datatroll::{Cell, Sheet};

fn main() {
    // Read data from a CSV file
    let mut sheet = Sheet::new();
    if let Err(err) = sheet.load_data("input.csv") {
        eprintln!("Error loading data: {}", err);
    } else {
        println!("Data loaded successfully from input.csv");
    }

    // drop all the rows in which the review is less than 4.0
    sheet.drop_rows("review", |c| {
        if let Cell::Float(r) = c {
            return *r < 4.0;
        }
        false
    });

    // calculate the variance of the review column
    let variance = sheet.variance("review").unwrap();
    println!("variance for review is: {variance}");
    
    // Write the transformed data to a new CSV file
    if let Err(err) = sheet.export("output.csv") {
        eprintln!("Error exporting data: {}", err);
    } else {
        println!("Data exported successfully to output.csv");
    }
}
```