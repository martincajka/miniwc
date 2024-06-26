use std::io::{self};

use unicode_segmentation::UnicodeSegmentation;

pub fn process_query(query: &str, input: &str) -> io::Result<String> {
    let mut result = String::new();

    for char in query.chars().skip(1) {
        let count = count(char, input)?;
        result.push_str(&format!("{} ", count));
    }

    Ok(result)
}

fn count(query: char, input: &str) -> io::Result<usize> {
    match query {
        'c' => Ok(count_bytes(input)),
        'w' => Ok(count_words(input)),
        'l' => Ok(count_lines(input)),
        'm' => Ok(count_chars(input)),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid query. Only 'c', 'w', 'l' and 'm' are allowed. You entered - '{}'",
                    query
                ),
            ));
        }
    }
}

fn count_bytes(text: &str) -> usize {
    text.len()
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn count_chars(text: &str) -> usize {
    UnicodeSegmentation::graphemes(text, true).count()
}

#[cfg(test)]
mod tests {

    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn byte_count_more_than_0() {
        let text = "a";

        assert_eq!(1, count_bytes(text));
    }

    #[test]
    fn byte_count_0_bytes() {
        let text = "";

        assert_eq!(0, count_bytes(text));
    }

    #[test]
    fn byte_count_new_line_carriage_return_tab() {
        let text = "\n\r\t";

        assert_eq!(3, count_bytes(text));
    }

    #[test]
    fn byte_count_empty_space() {
        let text = " ";

        assert_eq!(1, count_bytes(text));
    }

    #[test]
    fn words_count_empty_string() {
        let text = "";

        assert_eq!(0, count_words(text));
    }

    #[test]
    fn words_count_empty_spaces() {
        let text = "   ";

        assert_eq!(0, count_words(text));
    }

    #[test]
    fn words_count_1_word() {
        let text = "abc";

        assert_eq!(1, count_words(text));
    }

    #[test]
    fn words_count_1_word_with_extra_spaces_around() {
        let text = "  abc  ";

        assert_eq!(1, count_words(text));
    }

    #[test]
    fn words_count_more_than_1_word() {
        let text = "  abc  def ";

        assert_eq!(2, count_words(text));
    }

    #[test]
    fn words_count_more_than_1_word_separated_by_new_line() {
        let text = "abc\ndef";

        assert_eq!(2, count_words(text));
    }

    #[test]
    fn words_count_more_than_1_word_separated_by_tab() {
        let text = "abc\tdef";

        assert_eq!(2, count_words(text));
    }

    #[test]
    fn lines_count_empty_string() {
        let text = "";

        assert_eq!(0, count_lines(text));
    }

    #[test]
    fn lines_count_one_line() {
        let text = " abc  def  /t gh /r";

        assert_eq!(1, count_lines(text));
    }

    #[test]
    fn lines_count_two_lines_newline_separator() {
        let text = "abc\ndef";

        assert_eq!(2, count_lines(text));
    }

    #[test]
    fn lines_count_two_lines_newline_and_carriage_return() {
        let text = "abc\n\rdef";

        assert_eq!(2, count_lines(text));
    }

    #[test]
    fn lines_count_two_lines_newline_added_as_last_char() {
        let text = "abc\ndef\n";

        assert_eq!(2, count_lines(text));
    }

    #[test]
    fn lines_count_two_lines_newline_and_carriage_return_added_as_last_char() {
        let text = "abc\ndef\n";

        assert_eq!(2, count_lines(text));
    }

    #[test]
    fn lines_count_two_empty_lines() {
        let text = "\n\n";

        assert_eq!(2, count_lines(text));
    }

    #[test]
    fn chars_count_empty_string() {
        let text = "";

        assert_eq!(0, count_chars(text));
    }

    #[test]
    fn chars_count_ascii_chars() {
        let ascii_chars: String = (0..128).map(|i| char::from_u32(i).unwrap()).collect();

        assert_eq!(128, count_chars(&ascii_chars));
    }

    #[test]
    fn chars_count() {
        assert_eq!(count_chars("Hello, 世界!"), 10);
        assert_eq!(count_chars("👨‍👩‍👧‍👦"), 1);
        assert_eq!(count_chars("e\u{0301}"), 1);
    }

    #[test]
    fn count_using_c_option_counts_bytes() -> io::Result<()> {
        let input = "@#$%^&**():<>?\"hello /n} world";
        assert_eq!(count('c', input)?, count_bytes(input));
        Ok(())
    }

    #[test]
    fn count_using_w_option_counts_words() -> io::Result<()> {
        let input = "word/nword next /n/n word/tword/n";
        assert_eq!(count('w', input)?, count_words(input));
        Ok(())
    }

    #[test]
    fn count_using_l_option_counts_lines() -> io::Result<()> {
        let input = "word/nword next /n/n word/tword/n";
        assert_eq!(count('l', input)?, count_lines(input));
        Ok(())
    }

    #[test]
    fn count_using_m_option_counts_characters() -> io::Result<()> {
        let input = "世界/n👨‍👩‍👧‍👦/t/ne\u{0301}";
        assert_eq!(count('m', input)?, count_chars(input));
        Ok(())
    }

    #[test]
    fn test_count_invalid_query() {
        let result = count('x', "test input");
        assert!(matches!(result, Err(ref e) if e.kind() == ErrorKind::InvalidInput));
        assert!(
            matches!(result, Err(ref e) if e.to_string() == "Invalid query. Only 'c', 'w', 'l' and 'm' are allowed. You entered - 'x'")
        );
    }
}
