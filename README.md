# AutoTest: A Python Script Testing Utility

![image](https://github.com/kyborq/auto-testing/assets/52314985/17938525-cd2e-43d4-9106-783156e14b9f)

## Introduction

AutoTest is a lightweight utility designed for automated testing of Python scripts. It offers a straightforward way to validate the functionality of simple scripts, potentially providing a valuable tool for programming courses or individual practice. While it is currently optimized for Python, plans for future updates include support for additional programming languages.

## Features

- **Simple Test Creation:** Easily create test cases within a structured directory.
- **Automated Grading:** Ideal for programming assignments, allowing for automatic evaluation based on the number of passed tests.
- **Open Source:** Freely available for use, modification, and distribution

## Installation

To get started with AutoTest, clone the repository or download the latest release:

```bash
$ git clone https://github.com/kyborq/auto-testing.git
```

No additional installation steps are required. Ensure you have Python installed on your system to test Python scripts.

**Example File Structure:**

```
tests/
| test1.txt
| test1.expect.txt
| ...
| testN.txt
| testN.expect.txt
auto-test.exe
main.py
```

## Usage

1. **Prepare Test Cases:**
   - Within the same directory as your Python script, create a `tests` folder.
   - Add test files following the format: `test<number>.txt` for input and `test<number>.expect.txt` for the expected output.

2. **Run Tests:**
   - Place the `auto-test.exe` executable in the root directory alongside your script.
   - Execute the utility via the command line:
     ```
     ./auto-test.exe
     ```

## Limitations and Known Issues

1. **End-of-Line Marker Bug:** Ensure a newline character at the end of each test file to avoid issues with test execution.
2. **Language Support:** Currently limited to Python scripts. Future updates are planned to extend support to more programming languages.

## Contributing

We welcome contributions from the community! Whether it's adding new features, fixing bugs, or improving documentation, your help is appreciated.

