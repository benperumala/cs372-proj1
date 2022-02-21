# README

TODO Heavily document code

## Presentation
TODO Add link

## Rust Installation
Rust can be downloaded from their website <https://www.rust-lang.org/tools/install>  
If you have [Docker](https://www.docker.com/get-started), you can also use the `rust:1.58.1-buster` image and [mount the given projects](https://docs.docker.com/storage/bind-mounts/#start-a-container-with-a-bind-mount)

## Running Projects
Each "crate" must be compiled before run. This can be done via command line. `cd` into a given directory then execute `cargo run`. This will build the project then run it.

To get auto generated documentation (similar to javadocs, run `cargo doc`. This will generate documentation within `$PWD/target/doc/` where `$PWD` is the current working directory

## Project Descriptions

### Guessing Game
A simple game where the program picks a random number and you have to figure out what number it is. It will tell you if the number is lower or higher.

The rust implementation can be found within [guessing_game](guessing_game). To run the code, make sure you have rust installed. Then you can do `cargo run` within that directory to build and launch the program

A C implementation is also provided within [guessing_game_c](guessing_game_c). This is used to show the difference in workflow between Rust and C. To run this, make sure you have `gcc` installed. Then run `make compile && ./app` to launch the program.

### Fibonacci
[Link to sub-project](fibonacci)



### QueueBot
[Link to sub-project](queuebot)
An implementation of CS 120's QueueBot but within Rust instead of Python

### Web servers
