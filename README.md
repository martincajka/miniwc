# miniwc
![BuildApp](https://github.com/martincajka/miniwc/workflows/BuildApp/badge.svg)

miniwc is a command-line tool that provides functionality similar to the wc (word count) tool. It allows you to count bytes, words, lines, and characters in a file or from standard input.

## Usage
```bash
miniwc [options] [file]
```
If no file is provided, miniwc will read from standard input.

Options
`-c`: Count bytes
`-w`: Count words
`-l`: Count lines
`-m`: Count characters
If no options are provided, miniwc will use `-lwc` by default.

The result is always displayed in the order as the command-line arguments were provided.

Examples
Count lines, words, and bytes in a file:
```bash
miniwc -lwc file.txt
```
Count characters from standard input:
```bash
echo "Hello, world!" | miniwc -m
```

## Installation

To make your Rust application miniwc executable from the command line using the miniwc command, follow these steps:

Build your application in release mode by running `cargo build --release`. This creates an executable file in the target/release directory.

Add the target/release directory to your system's PATH. The command to do this varies depending on your shell:

For bash, add the following line to your ~/.bashrc or ~/.bash_profile file:
``` bash
export PATH="$PATH:/path/to/your/project/target/release"
```
For fish, add the following line to your ~/.config/fish/config.fish file:
``` fish
set -x PATH $PATH /path/to/your/project/target/release
```
For zsh, add the following line to your ~/.zshrc file:
``` zsh
export PATH="$PATH:/path/to/your/project/target/release"
```
Replace `/path/to/your/project` with the actual path to your project.

Open a new terminal window or source your shell configuration file to apply the changes. Now, you should be able to run your application using the miniwc command.

*Note*: This makes the miniwc command available for your user only. If you want to make it available system-wide, you need to move the executable to a directory that's in the system's PATH, like /usr/local/bin. You can do this with the command `cp target/release/miniwc /usr/local/bin/`, but you might need to use *sudo* to have the necessary permissions.
