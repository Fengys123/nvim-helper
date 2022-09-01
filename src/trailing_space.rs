pub fn trailing_space_statistics(lines: Vec<String>) -> Vec<usize> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| {
            if end_with_space(line) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .collect()
}

fn end_with_space(line: &str) -> bool {
    line.ends_with(' ')
}

#[cfg(test)]
mod tests {
    use super::end_with_space;

    #[test]
    fn test_end_with_space() {
        let s1 = "123 ";
        let s2 = " ";
        let s3 = " 23  ";
        let s4 = "123";

        assert_eq!(true, end_with_space(s1));
        assert_eq!(true, end_with_space(s2));
        assert_eq!(true, end_with_space(s3));
        assert_eq!(false, end_with_space(s4));
    }
}

