# rlox
Implementation of the Lox programming language from Robert Nystrom in Rust.

This implementation covers the first half of book and fully implements jlox from scanning to inheritance. I tried to find a balance between following the book and developing my own solutions, however, for the most part, this implementation should follow the book pretty closely.

### Technology Used
* [Rust](https://www.rust-lang.org/)

## Instructions/Getting Started

Use ```cargo run``` with no arguments to start the Lox interpreter in interactive prompt mode. Enter Lox code into the stdin prompt at the command line to execute it. Any output will be printed to stdout and the prompt will appear again.

Use ```cargo run -- path_to_lox_file``` to run Lox code from a file. The interpreter will execute the code and direct any output to stdout.

---

## Notices/Todo
* This has been the largest Rust project I have worked on so far (at the time of development at least). There were a lot of new concepts and structural things I learned, so in many places the code is not as clean as it could be, but it gets the job done. At some point I may come back and clean things up, but for now I want to move onto the Lox bytecode interpreter and other projects.
* Basic, manual testing has been done at all stages of development, so the code should work for most, if not all, cases. In the future, it would be nice to integrate the official Lox test suite.