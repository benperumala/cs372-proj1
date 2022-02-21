# README

## Github Repo
[Click me to go to github repository](https://github.com/benperumala/cs372-proj1)

## Presentation
[Link to Presentation Slide Deck](Presentation.pptx)
TODO Link to Video
[Link to Video]()

## Rust Installation
Rust can be downloaded from their website <https://www.rust-lang.org/tools/install>  
If you have [Docker](https://www.docker.com/get-started), you can also use the `rust:1.58.1-buster` image and [mount the given projects](https://docs.docker.com/storage/bind-mounts/#start-a-container-with-a-bind-mount)

## Running Projects
Each "crate" must be compiled before run. This can be done via command line. `cd` into a given directory then execute `cargo run`. This will build the project then run it.

To get auto generated documentation (similar to javadocs, run `cargo doc`. This will generate documentation within `$PWD/target/doc/` where `$PWD` is the current working directory. You can then open the `index.html` within that directory in a web browser.

## Project Descriptions

### Guessing Game
A simple game where the program picks a random number and you have to figure out what number it is. It will tell you if the number is lower or higher.

The rust implementation can be found within [guessing_game](guessing_game). To run the code, make sure you have rust installed. Then you can do `cargo run` within that directory to build and launch the program

A C implementation is also provided within [guessing_game_c](guessing_game_c). This is used to show the difference in workflow between Rust and C. To run this, make sure you have `gcc` installed. Then run `make compile && ./app` to launch the program.

### Fibonacci
[Link to sub-project](fibonacci)  
Two different implementations of fibonacci. One is a basic recursive definition. The other using dynamic programming paradigms to save previous results for future calls.

### QueueBot
[Link to sub-project](queuebot)  
An implementation of CS 120's QueueBot but within Rust instead of Python. See that project's README for more information

### Actix Web Server
[Link to sub-project](actix_example)  
Since I'm a web developer, I thought I would try one of Rust's HTTP server crates. This project defines an HTTP server (which can be viewed via a web browser). See the README within that project for more information.
