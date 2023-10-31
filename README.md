
---

# Design Patterns in Rust

This project demonstrates various design patterns implemented in Rust. Users can execute different patterns directly from the command line to see them in action.

## Getting Started

To run a design pattern, use the following command:

```bash
cargo run -- -n [PATTERN_NAME]
```

Replace `[PATTERN_NAME]` with the name of the design pattern you want to run.

If you'd like to execute the associated puzzle (if available) for a design pattern, simply add the `-p` option:

```bash
cargo run -- -n [PATTERN_NAME] -p
```

## Available Patterns

The following design patterns are currently implemented:

- `observer`
- `strategy` (includes an associated puzzle)

To run the puzzle for the `strategy` pattern, use the `-p` option as shown above.

Example:
```bash
cargo run -- -n strategy -p
```

## Command Line Options

Here's a quick overview of the available command line options:

```
Usage: design_patterns.exe [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>     Specify the name of the design pattern to run.
  -p, --run_puzzle      Optionally run the associated puzzle for the specified pattern.
  -h, --help            Print help information.
  -V, --version         Print version information.
```

## Contributing

If you find a bug or would like to suggest a new design pattern example, please open an issue or submit a pull request.

---
