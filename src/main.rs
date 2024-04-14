use miniwc::{run, Config};

use std::process;

fn main() {
    let conf = Config::build(std::env::args().skip(1));

    if let Err(e) = run(conf) {
        eprintln!("App error: {e}");
        process::exit(1);
    }
}
