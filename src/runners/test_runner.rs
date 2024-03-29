use std::path::Path;
use std::{fs, io};

use crate::utils::{get_test_files, run_program};

pub fn run_test(actual_output: &str, expected_output: &str) -> bool {
    actual_output.trim() == expected_output.trim()
}

pub fn print_result(output: &str, expected: &str, result: bool, number: i32) {
    print!("[{}] ", number);

    if result {
        print!("✅ Успешно ");
    } else {
        print!("❌ Ошибка ");
    }

    println!(
        "— Ожидал: {:?}, получил: {:?}",
        expected.trim(),
        output.trim()
    );
}

// pub fn run_single_test() -> io::Result<()> {
//     let test_input = fs::read_to_string("test.txt").expect("Failed to read test.txt");
//     let expected_output = fs::read_to_string("expect.txt").expect("Failed to read result.txt");

//     let actual_output = run_program(&test_input);
//     let result = run_test(&actual_output, &expected_output);

//     println!("------ Автотест v0.1.0 ------");
//     print_result(&actual_output, &expected_output, result, 1);

//     Ok(())
// }

pub fn run_all_tests(path: &Path) -> io::Result<()> {
    let test_files = get_test_files(path)?;
    let mut test_number = 1;
    let mut success_tests = 0;

    for entry in test_files {
        let test_input_path = entry.path();
        let test_input = fs::read_to_string(&test_input_path)?;

        let expected_output_path = test_input_path.with_extension("expect.txt");
        let expected_output = match fs::read_to_string(&expected_output_path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let actual_output = run_program(&test_input);
        let result = run_test(&actual_output, &expected_output);

        print_result(&actual_output, &expected_output, result, test_number);

        if result {
            success_tests += 1;
        }

        test_number += 1;
    }

    println!("------ Баллы: {} / {}", success_tests, test_number - 1);

    Ok(())
}
