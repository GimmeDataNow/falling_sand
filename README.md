# Re-Noita

Re-Noita is a 2D pixel / tile based game engine written in Rust.
### Motivation
Re-Noita is a game engine written in Rust that is meant to replicate the Falling Everything game engine upon which the game Noita is built.
Both the Falling Everything game engine and Noita were developed by Nolla Games. 

While playing the game, I would often encounter crashes or other game-breaking bugs when traveling far from the world's origin. The frequency and the severity of these bugs directly correlated to the distance from the world origin. As such, I have decided to recreate the Falling Everything game engine in Rust.
### Why Rust?
Rust is an awesome type safe language that forces you to resolve nearly all of your code issues at compile time. This means that the usual mess of debugging (I am oh so familiar with) is mostly handled before code execution. I personally hated receiving cryptic errors at runtime which were always a nightmare to debug, and Rust's approach of handling this is very appealing to me. This of course would never be this appealing without the help of the compiler. To be honest, the compiler may be the biggest attraction of Rust. It helps resolve any issues in your code not only by highlighting any mistakes but also offering suggestions on how to circumvent or resolve the issues the compiler has noticed. In addition, Rust is not a garbage collected language which I value greatly. A recent small project I had written in C# was constantly in flux when it came to memory, which left me with a sour tast in my mouth.

TLDR:
- type safe
- fast
- no garbage collection
- awesome compiler
- very little runtime issues
- errors mostly handled at compile time
- cross plattform

## Things i learned in this project
I pretty much learned most of what I know about rust when starting this project through trail and error (and the compiler) and by watching very short youtube videos explaining some topics briefly. I relied on the Intellisense of the rust-analyser extension to discover functions and methods that I now use on a regular basis.

I also learned to write better documentation using the '///# text' annotation above a function definition (Not all documentation is complete yet!). In addition I also learned how to use git. I previously never used it, but it became a necessity to manage several code branches at once without loosing any files.

## Things i aim to learn in this project

- how to utilise the gpu using vulcano
- how to write shaders
- how to render the gpu result to the screen without any libraries beside winit and vulcano
- how rasterise a mesh to a screen
- how to ray trace in a pixel / tile based engine
- how to properly utilize matricies with meshes

## How to build and run this code
There are several branches and you should pick one. The master branch is the least updated one, but also the most stable one. At some point in the future I may distribute the executables

```sh
git clone --branch <the branch that you chose> https://github.com/GimmeDataNow/falling_sand
cd /falling_sand
cargo build --release

// then to run the executable do
cargo run --release
```

## Bugs and other issues
I will generally ignore any issues on branches I am not actively working on because most of them will have been addressed in the most recent branch. But feel free to inform me of any issues by heading over to the issues tab.

## Credits
This is meant to be a solo project but if I receive any notable help then the names will be listed here:

## How to contribute to the project
As this is meant to be a solo project, I will be hesitant to accept any contributions. Regardless, you can create a pull request or an issue for me to look into.
