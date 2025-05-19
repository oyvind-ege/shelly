# Shelly
This is allegedly a POSIX-compliant shell, based on the codecrafters challenge available here:
[codecrafters.io](https://codecrafters.io).

In this challenge, I built my own shell capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, I learned about shell command parsing,
REPLs, builtin commands, and - above all - familiarized myself with Rust.

![Screenshot 2025-05-19 at 13 07 51](https://github.com/user-attachments/assets/cc404870-397a-4efc-a2c7-54d4a6a98f2c)

# Features

* Builtin commands:
  * exit
  * echo
  * type
  * pwd
  * cd
* Capable of running all commands available in PATH
* Supports single- and double quotes

## Perhaps in future:
* Redirects
