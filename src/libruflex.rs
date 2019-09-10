pub fn is_data_line(line: &str) -> bool {
    return line.starts_with("col") && line.len() > 3 && line.chars().nth(3).unwrap().is_numeric();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_lines() {
        assert!(is_data_line("col1"));
        assert!(!is_data_line("cola"));
        assert!(!is_data_line("xx"));
        assert!(!is_data_line(""));
    }
}
