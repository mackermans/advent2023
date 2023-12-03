pub fn run() {
    println!("Advent2023 day1");
    let input = include_str!("./input.txt");
    let parsed_input = parse_input(input);
    let sum = sum_all_digits(parsed_input);
    println!("1-1: {}", sum)
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut parsed_input: Vec<&str> = vec![];

    for line in input.lines() {
        parsed_input.push(line);
    }

    parsed_input
}

fn parse_line_digits(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn combine_first_last_digits(digits: Vec<u32>) -> u32 {
    let first = digits[0];
    let last = digits[digits.len() - 1];

    first * 10 + last
}

fn sum_all_digits(lines: Vec<&str>) -> u32 {
    lines.into_iter().fold(0, |aggregator, line| {
        let parsed_digits = parse_line_digits(line);
        let combined = combine_first_last_digits(parsed_digits);
        aggregator + combined
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_parse_line_digits(#[case] input: &str, #[case] expected_sum: u32) {
        let digits = parse_line_digits(input);
        let actual_sum = combine_first_last_digits(digits);
        assert_eq!(expected_sum, actual_sum);
    }

    #[test]
    fn test_sum_all_digits() {
        let text_input: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let parsed_input = parse_input(text_input);
        let sum = sum_all_digits(parsed_input);
        assert_eq!(sum, 142);
    }
}
