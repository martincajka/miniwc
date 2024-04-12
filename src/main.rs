use miniwc::{run, Config};

use std::process;

fn main() {
    let conf = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = run(conf) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}
