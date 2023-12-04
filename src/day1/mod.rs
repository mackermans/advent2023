pub fn run() {
    println!("Advent2023 day1");
    let input = include_str!("./input.txt");
    let parsed_input = parse_input(input);
    let sum = sum_all_digits(&parsed_input, false);
    println!("1-1: {}", sum);
    let sum_of_digits_and_number_words = sum_all_digits(&parsed_input, true);
    println!("1-2: {}", sum_of_digits_and_number_words);
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut parsed_input: Vec<&str> = vec![];

    for line in input.lines() {
        parsed_input.push(line);
    }

    parsed_input
}

const NUMBER_WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_first_digit(line: &str, from_start: bool, parse_words: bool) -> Option<u32> {
    let mut digit: Option<u32> = None;
    let mut digit_index: Option<u32> = None;
    let mut chars: Vec<char> = line.chars().collect();

    if !from_start {
        chars.reverse();
    }

    for (char_index, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            digit = c.to_digit(10);
            digit_index = Some(char_index.try_into().unwrap());
            break;
        }
    }

    if !parse_words {
        return digit;
    }

    for (word, word_digit) in NUMBER_WORDS {
        let needle: String = if from_start {
            word.chars().collect()
        } else {
            word.chars().rev().collect()
        };

        let haystack: String = chars.iter().collect();

        let word_index = match haystack.find(&needle) {
            Some(idx) => u32::try_from(idx).unwrap(),
            None => continue,
        };

        if digit_index.is_none() || word_index < digit_index.unwrap() {
            digit_index = Some(word_index);
            digit = Some(word_digit);
        };
    }

    digit
}

fn parse_line_digits(line: &str, parse_words: bool) -> Vec<u32> {
    let first_digit = get_first_digit(line, true, parse_words).unwrap();
    let last_digit = get_first_digit(line, false, parse_words).unwrap();
    vec![first_digit, last_digit]
}

fn combine_first_last_digits(digits: Vec<u32>) -> u32 {
    let first = digits[0];
    let last = digits[digits.len() - 1];

    first * 10 + last
}

fn sum_all_digits(lines: &[&str], parse_words: bool) -> u32 {
    lines.iter().fold(0, |aggregator, line| {
        let parsed_digits = parse_line_digits(line, parse_words);
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
    #[case("treb7uchetone", 77)]
    fn test_parse_line_digits(#[case] input: &str, #[case] expected_sum: u32) {
        let digits = parse_line_digits(input, false);
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
        let sum = sum_all_digits(&parsed_input, false);
        assert_eq!(sum, 142);
    }

    #[rstest]
    #[case("abc", true, None)]
    #[case("a1b2c3d4e5f", true, Some(1))]
    #[case("a1b2c3d4e5f", false, Some(5))]
    #[case("3two", true, Some(3))]
    #[case("five3twone", true, Some(5))]
    #[case("five3twone", false, Some(1))]
    fn test_first_digit_or_number_word(
        #[case] line: &str,
        #[case] from_start: bool,
        #[case] expected_digit: Option<u32>,
    ) {
        let actual_digit = get_first_digit(line, from_start, true);
        assert_eq!(expected_digit, actual_digit);
    }
}
