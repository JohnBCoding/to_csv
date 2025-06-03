//! [![github]](https://github.com/JohnBCoding/to_csv)&ensp;[![crates-io]](https://crates.io/crates/to_csv)&ensp;[![docs-rs]](https://docs.rs/to_csv/0.2.0/to_csv/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

use anyhow::{Error, Result};
use std::{fs::File, io::Write};

/// Implement this trait on the structs you will be passing to this libs functions.
pub trait CSV {
    /// Return a string of headers, comma separated.
    fn headers(&self) -> String;
    /// Return a string of comma separated values that will make up a row.
    fn row(&self) -> String;
}

/// Converts given Vec<T> that implements the CSV trait to a csv string.
pub fn to_csv_string<T: CSV>(entries: &Vec<T>) -> String {
    let mut csv_string = format!("{},\n", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},\n", csv_string, entry.row());
    });

    csv_string
}

/// Converts given Vec<T> that implements the CSV trait to a csv string that is URL encoded
pub fn to_csv_string_with_encode<T: CSV>(entries: &Vec<T>) -> String {
    let mut csv_string = format!("{},%0D%0A", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},%0D%0A", csv_string, entry.row());
    });

    csv_string
}

/// Converts given Vec<T> that implements the CSV trait to a csv string then saves it to the given file name
pub fn to_csv_file<T: CSV>(file_name: &str, entries: &Vec<T>) -> Result<(), Error> {
    let mut csv_string = format!("{},\n", entries[0].headers());

    entries.iter().for_each(|entry| {
        csv_string = format!("{}{},\n", csv_string, entry.row());
    });

    let mut file = File::create(file_name).unwrap();
    file.write_all(csv_string.as_bytes())?;
    Ok(())
}
