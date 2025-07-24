# Shelly - a poorly written shell
This is an atrociously written shell, in Rust :D. 

This was my first project in Rust, and I absolutely hate it. But it's fun to look at the trainwreck that is the parsing logic.

I am not very excited about parsing, abstract syntax trees, tokenization, etc. etc, either, so **unfortunately this project is no longer being worked on**.

## So, what can this thing do?

It prompts you for input, and it correctly handles most things like `echo` `type` `ls` - some simple builtins plus most of your own PATH executables.
It also supports and correctly interprets, by some miracle, both single- and double-quotes.

## What's the gist?
It uses a state-machine in the logic parser to determine how to interpret each additional character from the input. There is no fancy tokenization, no scalable architecture. Just pure and simply chaos that leads to a functional bare-bones Shell.
