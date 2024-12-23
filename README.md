# Forest Fire Sim

## About

This simulation shows a forest whose trees are constantly growing and then being 
burned down. On each step of the simulation, there is a 1 percent chance that a 
blank space grows into a tree and a 1 percent chance that a tree is struck by lightning 
and burns. Fires will spread to adjacent trees, so a densely packed forest is more likely 
to suffer a larger fire than a sparsely packed one. This simulation was inspired by 
Nicky Case’s Emoji Sim at http://ncase.me/simulating/model/.

## Running the project
* Install Rust: [rustup.rs](https://rustup.rs/)
* Clone the repository locally:
  * `git clone https:://github.com/AndrewRosenfrisk/forest-fire-sim`
  * `cd forest-fire-sim`
* Build the project with `cargo build`
* Run the project with `cargo run`

Based on the project detailed in the "[Big Book of Small Python Projects](https://inventwithpython.com/bigbookpython/project29.html)"
