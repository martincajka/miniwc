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
                    query: "-lwc".to_string(),
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
                query: "-lwc".to_string(),
                file: Some(first_arg),
            }
        }
    }
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
        assert_eq!(result.query, "-lwc");
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
        assert_eq!(result.query, "-lwc");
        assert!(result.file.is_none());
    }
}
