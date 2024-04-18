# miniwc
![BuildApp](https://github.com/martincajka/miniwc/workflows/BuildApp/badge.svg)
A simple word counter written in Rust.

## Usage

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
