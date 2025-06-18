# wnucore-utils 🦀

A personal implementation of core Unix utilities written in Rust, targeting **Windows** compatibility.  
Inspired by GNU Coreutils — but simpler, modern, and built for fun and extensibility.

---

## 🧰 Current Utilities

| Command | Description                             |
|---------|-----------------------------------------|
| `cat`   | Prints the contents of a file           |
| `clear` | Clears the terminal screen              |
| `grep`  | Searches for patterns in a file         |
| `head`  | Shows the first few lines of a file     |
| `lsw`   | Lists files in a directory (colorized)  |
| `rm`    | Deletes files or directories            |
| `tail`  | Shows the last few lines of a file      |
| `touch` | Creates new empty files                 |
| `wnu`   | Lists available wnucore-utils commands  |

---

## 📛 Name: wnucore-utils?

Yes! It stands for:

    Windows + GNU Coreutils = wnucore-utils
Not really creative but I like it


---

## 🛠 Usage

Each utility is implemented as a **separate binary** in the project.

Run them via Cargo like so:

```bash
cargo run --bin cat ./some_file.txt
cargo run --bin lsw ./some_folder
cargo run --bin grep "pattern" ./file.txt
cargo run --bin rm ./some_path
cargo run --bin head ./file.txt
cargo run --bin tail ./file.txt
```
You can also build all binaries for production use:
```bash
cargo build --release
```
They'll be output to:
```bash
./target/release/
```
## ➕ Add to $PATH

To use the commands globally (from any terminal):

1- Open File Explorer and go to the project’s target/release folder.

 2-   Copy the full path 

(e.g.,
  D:\wnucore-utils\target\release).

or

(D:\wnu-windows\) if you downloded the precompiled binaries

 3-   Open Environment Variables:

-    Win + S → search "Environment Variables"

-   Click "Edit the system environment variables"

-   Click "Environment Variables"

 4-   Under User variables, find Path → click Edit.

 5-   Click New, then paste the path to the release folder.

 6-   Click OK on all dialogs to save and apply.

Now you can run commands like:
```bash
cat file.txt
grep keyword file.txt
wnu
```
From anywhere in your terminal.

## 🧪 Testing

Tests exist for most commands and are functional for basic validation.

To run all tests:
```bash
cargo test
```
Tests are located in the corresponding src/bin/*.rs files and a few in ./tests/

    Most tests were scaffolded using AI and will be improved with more edge-case coverage and proper setup/teardown logic.

## 🤖 Acknowledgment

AI was used in this project, mainly for this readme file and the tests, but I also manually tested and wrote everything.

I am new to rust so this was a project to learn it with and of course with the help of AI