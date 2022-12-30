# Rusty-Pass

A command line password manager written in Rust. The (encrypted)passwords are stored in an SQLite database

## About

This is mostly a way for me to get more comfortable in Rust.

I hope to support the following features:

- [x] Generate a random password (with minute control over the length and various character groups)
- [x] Store encrypted passwords
- [x] Search for websites using SQLite Regular Expressions
- [x] Import/Export passwords

And that's about it.

## Running and Developing

This probably goes without saying, but here's the steps anyways - Simply clone the project, and then do

```bash
cargo build --release
```

The generated binary is `target/release/rusty-pass`. Run it with `--help` to see the available commands.
