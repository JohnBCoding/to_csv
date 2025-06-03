This library provides a lightweight way to export data to the CSV format via trait implementation.

```toml
[dependencies]
to_csv = "0.2"
```

## Example

```rust
impl CSV for TypeA {
    fn headers(&self) -> String {
        format!("{},{},{}", "Data Name", "Amount", "Date",)
    }
    fn row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.name,
            self.value,
            self.date,
        )
    }
}
```

```csv
Data Name,Amount,Date
Test Data,10,6/3/2025
```
