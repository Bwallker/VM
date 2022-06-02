# Myvm

VM and assembler loosly inspired by the OVERTURE architecture featured in the game Turing Complete.

## Overview
This repo is divided into 2 parts. The original parser and runtime written in Rust, and the website that uses the orignal parsers alongside its own runtime in order to dynamically run your programs.

## Use Cases
Might be useful for learning Assembler, or for learning about how ASCII/Utf-8 work.

I created this mostly for fun.

## Website

### Features
- a Code editor powered by monaco-editor with syntax highlighting.
- Save and load your programs to and from localStorage.
- Includes several showcase programs that shows off the assemblers features, and shows how the runtime works.
- Half finished Docs.
- View the generated bytecode for your program, aswell as what it looks like after macro expansion. 
- Run your programs, and step through them one instruction at a time.
- You can dynamically give input to your program through the STDIN feature which emulates STDIN.
- Uses UTF-8 as the output, which means that you can print emoji.

## Rust native assembler and runtime:

### Features
- Assemble, Assemble and run, or Run your programs.
- Use dynamic input with stdin or read your input from a file or directly from the program.
- Runtime is much faster.
- Haven't tested emoji support. Might not work.


## TODO

Here are the some things that are yet to be added/fixed. Pull requests are welcome for these things or anything else you would like to add.
- Add debugger to website so can insert breakpoints and view which row in your source code is currently being executed.
- Replace favicon with a proper logo.
- Fix needing a newline after a comment if it's the last thing in your program. (Lexer bug, should be a fairly easy change in grammar.pest)
- Improve performance of TS runtime. Currently no real effort has been put into making it fast.
- Add better error messages to the parser.
