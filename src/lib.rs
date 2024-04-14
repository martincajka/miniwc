use std::{
    fs,
    io::{self, Read},
};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Config {
        args.next();
        let first_arg = match args.next() {
            Some(arg) => arg,
            None => {
                return Config {
                    query: "-cwl".to_string(),
                    file: None,
                }
            }
        };

        if first_arg.starts_with('-') {
            match args.next() {
                Some(arg) => Config {
                    query: first_arg,
                    file: Some(arg),
                },
                None => Config {
                    query: first_arg,
                    file: None,
                },
            }
        } else {
            Config {
                query: "-cwl".to_string(),
                file: Some(first_arg),
            }
        }
    }
}

pub fn run(config: Config) -> io::Result<()> {
    let content = match &config.file {
        Some(file) => fs::read_to_string(file)?,
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    for char in config.query.chars().skip(1) {
        match char {
            'c' => {
                let bytes = count_bytes(&content);
                println!("Bytes: {}", bytes);
            }
            'w' => {
                let bytes = count_words(&content);
                println!("Words: {}", bytes);
            }
            'l' => {
                let bytes = count_lines(&content);
                println!("Lines: {}", bytes);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid query. Only 'c', 'w', 'l' are allowed",
                ))
            }
        }
    }

    Ok(())
}

fn _count(c: &char, text: &str) -> Result<usize, &'static str> {
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
        assert_eq!(result.query, "-cwl");
    }

    #[test]
    fn succesful_byte_count() {
        let text = "abc";

        assert_eq!(3, count_bytes(text));
    }
}
