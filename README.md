# CLI Payments Engine

A simple engine to parse and evaluate transaction CSV files.

## Usage

Simply run the engine with the file to parse as the sole argument and write the program's output to a file.

`$ cargo run -- transactions.csv > accounts.csv`

The output file will contain all the accounts after the transactions described.

Test cases were created within the `tesc_cases` folder, which can be run with `cargo test`.

Test cases can be added by adding files named `$name_input.csv` and `$name_output.csv` to the `test_cases` directory and adding the test identifier to the semicolon-separated list of test names in the tests module, _ie_, `gen_tests`.
By specifying a pattern to match against an `Error`, a failure will be expected instead of a success.
Example usage of `tests::gen_tests`:

```rust
gen_tests! {
    test_no_fail; // tests files test_no_fail_input.csv and test_no_fail_output.csv
    test_no_fail2;
    test_fail_any, _ = err;
    test_fail_parse, Error::Parse(_) = err;
    test_fail_runtime, Error::Runtime(_) = err;
}
```

The output file does not have to exist if the test expects an error.

Make sure to use LF for test cases and not CRLF, as the program outputs only LF.

## Errors

Error handling in this program was achieved using specialized structures and enums to hierarchically describe the errors by where in the program they occur.
At the crate's root is the `Error` type, which represents any error that can occur during execution.
The `Error` type is divided into parsing errors, runtime errors, and miscellaneous/other errors.
While the parsing errors and runtime errors are their own types, miscellaneous errors are simply represented by a string message.

All error types have `std::fmt::Display` implemented, as such it is necessary for the main function to be a wrapper of a seperate function, called `run` for simplicity's sake.
This wrapper prints the error using `eprintln` and exits with a code of 1.
Returning the `Result` type directly from main causes the error to be printed using the debug formatter instead of the display formatter.

## Command Pattern

This program sees the transaction CSV file as a list of commands with arguments to execute sequentially.
As such, the type representing a single row of the CSV file is called a `PaymentCommand`.
This pattern is easily extensible by simply adding more variants to the `PaymentCommand` enum and completing all of the match arms.
