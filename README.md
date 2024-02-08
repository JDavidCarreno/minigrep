# Command Line Project in Rust

This project is an exercise extracted from the book "The Rust Programming Language". It consists of a command-line program developed in Rust. Although it is quite simple, it is designed to put into practice the concepts learned up to Chapter 11 of the book.

## Objective
The main objective of this project is to implement and apply the topics studied in the book, including error handling, lifetimes, automated tests, among others. Through this exercise, we aim to strengthen the understanding and application of these concepts in a practical context.

## About Minigrep
Minigrep is a command-line program inspired by the `grep` command-line utility in Unix systems. The main function of Minigrep is to search for matches of a specific string in a given text file. Although simplified compared to `grep`, Minigrep follows the same basic principle of pattern searching in text files.

## Functionality
The user provides a search term and a text file as arguments to the program. Minigrep searches for all lines in the file that contain the search term and displays those lines to the standard output.

## Note
At the moment, this project is implemented exactly as it is described in the book. However, the goal is to continue learning and improving it by adding new features and enhancements.

## Example Usage
To use Minigrep, run the program from the command line as follows:

```sh
cargo run <search_term> <text_file>
```

For example:

```sh
cargo run "pattern" file.txt
```

This command will search for all lines in the file `file.txt` that contain the term "pattern" and display those lines to the standard output.

To perform a case-insensitive search, you can use the following variant of the command:

```sh
IGNORE_CASE=1 cargo run <search_term> <text_file>
```

For example:

```sh
IGNORE_CASE=1 cargo run "pattern" file.txt
```

This command will search for all lines in the file `file.txt` that contain the term "pattern", regardless of whether the letters are uppercase or lowercase.
