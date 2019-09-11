pub fn is_data_line(line: &str) -> bool {
    return line.starts_with("col") && line.len() > 3 && line.chars().nth(3).unwrap().is_numeric();
}

pub fn get_elements(line: &str) -> Vec<&str> {
    line.trim_start_matches("col")
        .split_whitespace()
        .collect::<Vec<&str>>()[..3]
        .to_vec()
}

pub trait IntConv {
    fn to_int(&self) -> Vec<i32>;
}

impl IntConv for Vec<&str> {
    fn to_int(&self) -> Vec<i32> {
        self.iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
}

pub fn parse_line(line: &str) -> Vec<i32> {
    get_elements(line).to_int()
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
    fn test_missing_elemtents() {
        get_elements("col1 1");
    }

    #[test]
    #[should_panic]
    fn test_element_not_numeric() {
        get_elements("col1 1 s").to_int();
    }

    #[test]
    fn test_parse() {
        assert_eq!(vec![1, 1, 1], parse_line("col1 1 1"));
        assert_eq!(vec![1, 1, 1], parse_line("1 1 1"));
    }
}
