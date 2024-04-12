use std::{default, error::Error, fmt::format, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path"),
        };

        Ok(Config {
            query: first_arg,
            file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.file_path)?;
    let count = count(&'c', &text);
    if let Ok(result) = count {
        println!("{} {}", result, &config.file_path);
    }

    Ok(())
}

fn count(c: &char, text: &str) -> Result<usize, &'static str> {
    match c {
        'c' => Ok(count_bytes(text)),
        'w' => Ok(count_words(text)),
        'l' => Ok(count_lines(text)),
        'm' => Ok(count_chars(text)),
        _ => Err("Error"),
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
    fn valid_args_byte_count_file_path() {
        let args = vec!["text.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing arguments");
    }

    #[test]
    fn succesful_byte_count() {
        let text = "abc";

        assert_eq!(3, count_bytes(text));
    }
}
