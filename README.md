# Rush 🐚

> A Unix-like shell written from scratch in Rust.

<!-- Optional badges — remove if you don't have CI/crates.io set up yet
[![Build Status](https://github.com/yourname/rush/actions/workflows/ci.yml/badge.svg)](https://github.com/yourname/rush/actions)
[![Crates.io](https://img.shields.io/crates/v/rush.svg)](https://crates.io/crates/rush)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
-->

`rush` is a POSIX-inspired command-line shell built from the ground up in Rust, as a project to learn how shells actually work under the hood — process control, parsing, job management, and I/O redirection — without leaning on an existing shell implementation.

## Features

- [x] Interactive REPL with prompt and line editing
- [x] Command execution via `fork`/`exec`
- [x] Built-in commands (`cd`, `exit`, `pwd`, `echo`, ...)
- [x] I/O redirection (`>`, `>>`, `<`)
- [x] Pipelines (`cmd1 | cmd2 | cmd3`)
- [ ] Background jobs (`&`) and job control (`jobs`, `fg`, `bg`)
- [ ] Environment variable expansion (`$VAR`)
- [ ] Command history and reverse search
- [ ] Tab completion
- [ ] Custom shell scripting / config file support (`.rushrc`)

> Adjust the checklist above to match what's actually implemented — it doubles as a quick roadmap for readers.

## Demo

```
$ rush
rush> echo "hello, world" | tr a-z A-Z
HELLO, WORLD
rush> ls -la > files.txt
rush> cd ~/projects
rush> exit
```

<!-- Consider replacing this with an actual terminal recording (asciinema / a gif via vhs) -->

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later) and Cargo

### Build from source

```bash
git clone https://github.com/yourname/rush.git
cd rush
cargo build --release
```

The compiled binary will be at `target/release/rush`.

### Run it

```bash
cargo run
# or, after building:
./target/release/rush
```

### Install locally

```bash
cargo install --path .
```

## Usage

Once running, `rush` behaves like a standard shell:

```bash
rush> pwd
rush> ls -l | grep ".rs"
rush> echo $HOME
rush> cat input.txt > output.txt
```

Run `help` inside the shell to see all supported built-ins (if implemented).

## Architecture

A brief overview of how the shell is structured, to help contributors (and future you) navigate the code:

```
src/
├── main.rs          # Entry point, REPL loop
├── parser/          # Tokenizing and parsing input into an AST/command struct
├── executor/         # Process spawning, pipes, redirection, job control
├── builtins/         # Built-in command implementations (cd, exit, etc.)
└── shell.rs          # Shell state (cwd, env vars, history)
```

**Pipeline for a typed command:**

1. **Read** — input line captured from stdin
2. **Parse** — tokenized and turned into a command structure (handling pipes, redirects, quoting)
3. **Execute** — built-ins are run in-process; external commands are `fork`/`exec`'d, with pipes and file descriptors wired up as needed
4. **Loop** — wait on the process(es), print output, return to prompt

> Replace this section with your actual module layout — this is just a common shape.

## Roadmap

- [ ] POSIX-compliant scripting support
- [ ] Signal handling (`Ctrl+C`, `Ctrl+Z`)
- [ ] Alias support
- [ ] Configurable prompt (PS1-style)

## Contributing

Contributions, issues, and feature requests are welcome.

1. Fork the repo
2. Create a branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push and open a Pull Request

Please run `cargo fmt` and `cargo clippy` before submitting.

## Testing

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE) — see the file for details.

## Acknowledgements

- Inspired by the classic [Build Your Own Shell](https://craftinginterpreters.com/) style learning projects and Unix shell internals (`dash`, `bash`, `fish`).
