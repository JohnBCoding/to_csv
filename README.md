This library provides a lightweight way to export data to the CSV format via trait implementation.

[<img alt="crates.io" src="https://img.shields.io/crates/v/to_csv.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/to_csv)
![](https://img.shields.io/crates/d/to_csv)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-to_csv-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/to_csv)

## Example

```toml
[dependencies]
to_csv = "1.0"
```

```rust
struct TypeA {
    name: String,
    value: String,
    date: String,
}

impl CSV for TypeA {
    fn headers(&self) -> String {
        format!("{},{},{}", "Data Name", "Amount", "Date",)
    }
    
    fn row(&self) -> String {
        format!(
            "{},{},{}",
            self.name,
            self.value,
            self.date,
        )
    }
}

fn main() {
    let entries = vec![TypeA {name: "Test Data".to_string(), value: "10".to_string(), date: "6/3/2025".to_string()}]
    let _ = to_csv_file("csv_file.csv", &entries);
}
```

## Result

```csv
Data Name,Amount,Date,
Test Data,10,6/3/2025,
```
