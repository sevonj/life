[![CI](https://github.com/sevonj/life/actions/workflows/rust.yml/badge.svg)](https://github.com/sevonj/life/actions/workflows/rust.yml)
___
# Life

A game that resembles The Sims.

https://github.com/user-attachments/assets/97b84145-440b-446f-bb0d-8186cdd1704a

## Project Goals

This project is an experiment that aims to explore a few things:

### Rust bindings for Godot game engine

GDScript lacks some features and tools that one would expect from a "real" programming language. Development of complex systems has been tedious as many changes resulted in a regression. This is the second project in series of trying out C++, Rust, and C# as a GDScript replacement.

All game code shall be written in Rust, with possible exception of GDScript for UI elements or other minor things.  
  
### Complex decision making with utility AI

(Traditional game AI, built with logic. Not the machine learning kind of AI)

My previous hierarchial state machine (modeled after fundamentals of [Source Engine AI](https://developer.valvesoftware.com/wiki/Category:AI)) ended up working great, but [The Sims offers some very interesting conceps](https://gmtk.substack.com/p/the-genius-ai-behind-the-sims), such as _needs,_ and _advertisements._

### Procedural mesh generation

Building a home in the game involves procedurally generating floors, walls, staircases, etc. and further modifying the result by for example cutting openings for doors and windows, making this kind of a spiritual successor to [WorldEdit](https://github.com/sevonj/worldedit).

## Building

**Requirements:**  
- [Godot 4.3](https://godotengine.org/)
- [Rust programming language](https://www.rust-lang.org/)

**Steps:**  
- Run `cargo build` in the rust directory
- Open the project in Godot editor

## License
TODO - Probably GPL.

Also, some files fit for generic reuse are _in addition_ available under more permissive MPL 2.0. It is mentioned in the file, if so.

## Continuous Integration

Pull requests are gatekept by [this workflow.](https://github.com/sevonj/life/blob/master/.github/workflows/rust.yml) It will check if the code

- ~~passes unit tests~~ (run `cargo test`)
- has linter warnings (run `cargo clippy`)
- is formatted (run `cargo fmt`)
