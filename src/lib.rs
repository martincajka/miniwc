use std::{
    fs,
    io::{self, Read},
};

pub mod config;
mod query;

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

pub fn run(config: config::Config) -> io::Result<String> {
    let input = read_input(&config)?;
    let output = query::process_query(&config.query, &input)?;

    println!("{} {}", output, config.file.unwrap_or("".to_string()));

    Ok(output)
}
