use comfy_table::Table;
use comfy_table::presets::NOTHING;

/// Build a table with no borders, mimicking `column -t`.
pub fn build_table(headers: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut table = Table::new();
    table.load_preset(NOTHING);
    table.set_header(headers);

    for row in rows {
        table.add_row(row);
    }

    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_with_headers_and_rows() {
        let rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        let output = build_table(&["Name", "Age"], rows);
        assert!(output.contains("Name"));
        assert!(output.contains("Age"));
        assert!(output.contains("Alice"));
        assert!(output.contains("30"));
        assert!(output.contains("Bob"));
        assert!(output.contains("25"));
    }

    #[test]
    fn table_with_empty_rows() {
        let output = build_table(&["Name", "Age"], vec![]);
        assert!(output.contains("Name"));
        assert!(output.contains("Age"));
        // No data rows
        assert!(!output.contains("Alice"));
    }
}
