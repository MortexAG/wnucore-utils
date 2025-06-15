# wnucore-utils 🦀

A personal implementation of core Unix utilities written in Rust, targeting Windows compatibility. Inspired by GNU Coreutils — but simpler, and with future extensibility.

---

## 🧰 Current Utilities

| Command | Description                        |
|---------|------------------------------------|
| `cat`   | Prints the contents of a file      |
| `lsw`   | Lists files in a directory (colorized) |
| `grep`  | Searches for patterns in a file    |
| `touch` | Creates new empty files            |
| `clear` | Clears the terminal screen         |

Planned next: `rm`, `head`, `tail`

---

## 📛 Name: wnucore-utils?

Yes! It stands for:

    Windows + GNU Coreutils = wnucore-utils
Not really creative but I like it

## 🛠 Usage

Each command is implemented as a separate binary in the workspace.
Example:
```bash 
cargo run --bin cat ./some_file.txt
cargo run --bin lsw ./some_folder
cargo run --bin grep "pattern" ./file.txt.
```

You can also build the project and move the binaries into your `$PATH` for real CLI-style usage (THIS IS NOT TESTED YET)

## 🧪 Testing

Most of the tests were written with the help of an AI assistant (me!).  
They're functional and useful for catching regressions — but will be improved with better edge case coverage soon.

To run tests:
```bash
cargo test
```
## 🤖 Acknowledgment

Some parts of this repo (notably tests and CLI scaffolding) were generated with the help of AI.
You can expect improvements and refactors soon — especially for test coverage and argument parsing.