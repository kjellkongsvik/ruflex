pub fn is_data_line(line: &str) -> bool {
    return line.starts_with("col") && line.len() > 3 && line.chars().nth(3).unwrap().is_numeric();
}

pub fn parse_line(line: &str) -> Vec<&str> {
    line.trim_start_matches("col")
        .split_whitespace()
        .collect::<Vec<&str>>()[..3]
        .to_vec()
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

    #[test]
    #[should_panic]
    fn test_should_panic() {
        parse_line("col1 1");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![1, 1, 1],
            parse_line("col1 1 1")
                .iter()
                .map(|x| x.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        );
        assert_eq!(vec!["1", "1", "1"], parse_line("1 1 1"));
    }
}
