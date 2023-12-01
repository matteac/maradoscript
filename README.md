# MaradoScript
<p align="center">
  <img src="images/maradona.jpeg" alt="Logo">
</p>
Inspired by Brainfuck and Maradona, written in Rust (btw)

## How it works
It has a memory of 2^16 bytes and a pointer that points to the current byte, you can move the pointer forward or backward and increment or decrement the value at the pointer, print to the stdout and take input

Be careful! You can underflow and overflow the memory AND the pointer

If the pointer is at zero and you try to decrement it, it will go to the last byte


## Instructions
- e - moves the pointer forward
- i - moves the pointer backward 
- m - increments the value at the pointer 
- a - decrements the value at the pointer
- 👍 - prints the value at the pointer to the stdout
-   (space) - takes an ASCII character from the stdin and stores it at the pointer


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
# or run use the repl
cargo run --realease --
```

## Example
This prints `Hello World`
```
mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm👍emmmmmmmmmm👍emmmmmmmmmmmmm👍
```
