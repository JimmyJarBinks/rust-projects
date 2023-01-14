# Super Pong

*Super Pong* is my first project using the Rust programming language. Upon discovering Rust in 2022, reading through an abundance of documentation and tutorials, and experimenting with smaller scripts of my own, I began exploring the application of Rust for game development. I ultimately settled on using the [Tetra] framework for this project due to its accessibility. This project began with the [official Tetra tutorial] and a subsequent desire to expand on it. Compared to the original tutorial, *Super Pong* adds the following:
* Multiple scenes including the title and pause screens
* 2 selectable game modes: Player vs. Player and Player vs. Computer
* 5 colors for each player's paddle
* A first-to-10 scoring system to allow for the game to end and be replayed.
* Particles, sound effects, and more!

Credit to [Kenney.nl] for visual and audio assets.

[Tetra]: https://tetra.seventeencups.net/
[official Tetra tutorial]: https://tetra.seventeencups.net/tutorial
[Kenney.nl]: https://www.kenney.nl/

## Images
 *-INSERT-IMAGE-HERE-*
 
## How to Play
In addition to the files in this directory, you will need SDL 2.0 and include ```SDL2.dll``` in the project directory.

Build the project with the following command: ```cargo build --release```

Run the game with the following command: ```cargo run```
