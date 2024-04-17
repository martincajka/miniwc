use std::{
    fs,
    io::{self, Read},
};

pub mod config;

fn read_input(config: &config::Config) -> io::Result<String> {
    match &config.file {
        Some(file) => fs::read_to_string(file),
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Ok(input)
        }
    }
}

fn process_query(config: &config::Config, input: &String) -> io::Result<String> {
    let mut result = String::new();

    for char in config.query.chars().skip(1) {
        let count = count(char, input)?;
        result.push_str(&format!("{} ", count));
    }

    Ok(result)
}

fn count(query: char, input: &String) -> io::Result<usize> {
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

pub fn run(config: config::Config) -> io::Result<String> {
    let input = read_input(&config)?;
    let output = process_query(&config, &input)?;

    println!("{} {}", output, config.file.unwrap_or("".to_string()));

    Ok(output)
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
