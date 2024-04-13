use miniwc::{run, Config};

use std::process;

fn main() {
    let conf = Config::build(std::env::args());

    if let Err(e) = run(conf) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}
