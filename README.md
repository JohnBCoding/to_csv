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
Organization,Network Name,Client Type,Client Description,Client Manufacturer,Client IP,Client MAC,Client Port,
Focus Brands Corporate,Focus Brands RT Lab,DMZ Device,,TRENDnet,10.10.6.4,3c:8c:f8:ff:52:c5,6,
```
