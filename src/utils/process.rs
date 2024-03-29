use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn run_program(test_input: &str) -> String {
    let mut child = Command::new("python")
        .arg("main.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run script");

    if let Some(ref mut stdin) = child.stdin {
        stdin
            .write_all(test_input.as_bytes())
            .expect("Failed to write to stdin");
    } else {
        panic!("Failed to open stdin");
    }

    let mut output = String::new();
    if let Some(ref mut stdout) = child.stdout {
        stdout
            .read_to_string(&mut output)
            .expect("Failed to read stdout");
    } else {
        panic!("Failed to open stdout");
    }

    output
}
