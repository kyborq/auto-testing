mod runners;
mod utils;

use std::{io, path::Path};

fn main() -> io::Result<()> {
    println!("------ Автотест v0.1.0");
    runners::run_all_tests(Path::new("tests"))
}
