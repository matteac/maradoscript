# MaradoScript
<p align="center">
  <img src="images/maradona.jpeg" alt="Logo">
</p>
Inspired by Brainfuck and Maradona, written in Rust (btw)

## How it works
This language operates with a memory of 2^16 bytes and a pointer that navigates the current byte. You can manipulate the pointer, move it forward or backward, increment or decrement the value at the pointer, print to the stdout, and take input.

Be cautious! It's possible to underflow or overflow both the memory and the pointer. If the pointer is at zero and an attempt is made to decrement it, it will go to the last byte.


## Instructions
- e - moves the pointer forward
- i - moves the pointer backward 
- m - increments the value at the pointer 
- a - decrements the value at the pointer
- 👍 - prints the value at the pointer to the stdout
- ' ' (space) - takes an ASCII character from the stdin and stores it at the pointer


## Usage
Clone the repo
```shell
git clone https://github.com/matteac/maradoscript.git
```
Build the project
```shell
cargo build --release
```
Run the project
```shell
cargo run --release -- -i <FILE>
```
Or use the repl
```shell
cargo run --release --
```

## Example
The following code prints `Hello World`:
```
mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmm👍emmmmmmmmmmmmm👍
```
