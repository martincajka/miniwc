use std::io::{self};

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
    text.len()
}

pub fn count_lines(text: &str) -> usize {
    text.len()
}

pub fn count_chars(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn succesful_byte_count() {
        let text = "abc";

        assert_eq!(3, count_bytes(text));
    }
}
