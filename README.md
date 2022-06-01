# Martian Robots

This is a solution to the Martian Robots code challenge.

## Getting started

Build the binary: `cargo build --release`

Execute the binary: ```target/release/robots --help```

Output:

> There is an example input file in the resources directory

```
rover 0.1.0
Martian rovers developer exercise

USAGE:
    robots -f <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f <file>        Path to the input file of rover instructions
```

## Development

The code has been written in rust (version 1.59.0). The following dependencies have been used:

- structopt (parsing command line input)
- nom (parsing the input file of instructions)

Build and run the binary locally:

```cargo run -- -f resources/input.txt```

#### Running tests

The tests can be run by executing:
```cargo test```

Sample test output:

```
Finished test [unoptimized + debuginfo] target(s) in 0.56s
Running unittests (target/debug/deps/robots-367505bd787c73b3)

running 13 tests
test bounding_box::test::test_inside ... ok
test coordinate::test::test_create ... ok
test houston::test::test_has_scent ... ok
test orientation::test::test_turn_left ... ok
test orientation::test::test_turn_right ... ok
test parser::test::test_parse_coordinate ... ok
test parser::test::test_parse_houston ... ok
test parser::test::test_parse_instruction ... ok
test parser::test::test_parse_number ... ok
test parser::test::test_parse_orientation ... ok
test rover::test::test_forward ... ok
test rover::test::test_turn_left ... ok
test rover::test::test_turn_right ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Code styling and linting

Format the code: `rust fmt`

Lint the code: `cargo clippy`
