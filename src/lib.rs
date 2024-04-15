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

fn read_input(config: &Config) -> io::Result<String> {
    match &config.file {
        Some(file) => fs::read_to_string(file),
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Ok(input)
        }
    }
}

pub fn run(config: Config) -> io::Result<()> {
    let input = read_input(&config)?;

    for char in config.query.chars().skip(1) {
        match char {
            'c' => {
                let bytes = count_bytes(&input);
                println!("Bytes: {}", bytes);
            }
            'w' => {
                let bytes = count_words(&input);
                println!("Words: {}", bytes);
            }
            'l' => {
                let bytes = count_lines(&input);
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
    fn parsing_cli_args_option_c_and_file() {
        let args = vec!["-c".to_string(), "test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-c");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_w_and_file() {
        let args = vec!["-w".to_string(), "test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-w");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_l_and_file() {
        let args = vec!["-l".to_string(), "test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-l");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_m_and_file() {
        let args = vec!["-m".to_string(), "test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-m");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_cwlm_and_file() {
        let args = vec!["-cwlm".to_string(), "test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-cwlm");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_only_file() {
        let args = vec!["test.txt".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-cwl");
        assert_eq!(result.file.expect("Filename is None"), "test.txt");
    }

    #[test]
    fn parsing_cli_args_option_only_query_arg() {
        let args = vec!["-c".to_string()].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-c");
        assert!(result.file.is_none());
    }

    #[test]
    fn parsing_cli_args_option_no_args() {
        let args = vec![].into_iter();

        let result = Config::build(args);
        assert_eq!(result.query, "-cwl");
        assert!(result.file.is_none());
    }

    #[test]
    fn succesful_byte_count() {
        let text = "abc";

        assert_eq!(3, count_bytes(text));
    }
}
