//! [![github]](https://github.com/JohnBCoding/to_csv)&ensp;[![crates-io]](https://crates.io/crates/to_csv)&ensp;[![docs-rs]](https://docs.rs/to_csv/0.2.0/to_csv/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

//! This library provides a lightweight way to export data to the CSV format via trait implementation.

//! ```toml
//! [dependencies]
//! to_csv = "1.0"
//! ```

//! ## Example

//! ```rust
//! impl CSV for TypeA {
//!     fn headers(&self) -> String {
//!         format!("{},{},{}", "Data Name", "Amount", "Date",)
//!     }
//!     fn row(&self) -> String {
//!         format!(
//!             "{},{},{}",
//!             self.name,
//!             self.value,
//!             self.date,
//!         )
//!     }
//! }

//! fn main() {
//!     let _ = to_csv_file("csv_file.csv", &entries);
//! }
//! ```

//! ## Result

//! ```csv
//! Data Name,Amount,Date,
//! Test Data,10,6/3/2025,
//! ```

use anyhow::{Error, Result, anyhow};
use std::{fs::File, io::Write};

/// Implement this trait on the structs you will be passing to this libs functions.
pub trait CSV {
    /// Return a string of headers, comma separated.
    fn headers(&self) -> String;
    /// Return a string of comma separated values that will make up a row.
    fn row(&self) -> String;
}

/// Converts given Vec<T> that implements the CSV trait to a csv string.
pub fn to_csv_as_string<T: CSV>(entries: &Vec<T>) -> Result<String, Error> {
    if entries.is_empty() {
        return Err(anyhow!("Entries vec is empty"));
    }

    let mut csv_string = format!("{},\n", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},\n", csv_string, entry.row());
    });

    Ok(csv_string)
}

/// Converts given Vec<T> that implements the CSV trait to a csv string that is URL encoded
pub fn to_csv_as_string_with_encode<T: CSV>(entries: &Vec<T>) -> Result<String, Error> {
    if entries.is_empty() {
        return Err(anyhow!("Entries vec is empty"));
    }

    let mut csv_string = format!("{},%0D%0A", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},%0D%0A", csv_string, entry.row());
    });

    Ok(csv_string)
}

/// Converts given Vec<T> that implements the CSV trait to a csv string then saves it to the given file_path
pub fn to_csv_as_file<T: CSV>(file_path: &str, entries: &Vec<T>) -> Result<(), Error> {
    if entries.is_empty() {
        return Err(anyhow!("Entries vec is empty"));
    }

    let mut csv_string = format!("{},\n", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},\n", csv_string, entry.row());
    });

    let mut file = File::create(file_path).unwrap();
    file.write_all(csv_string.as_bytes())?;
    Ok(())
}
