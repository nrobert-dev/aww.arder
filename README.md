# Welcome to Aww.arder!💬

Hi! **Aww.arder** is a simple, fun web-app written in [Rust](https://www.rust-lang.org/) and [Yew](https://github.com/yewstack/yew) which allows you and your team to offer praise to each other during your regular meetings. Create a team, add members to it and then compliment each other!

**Aww.awarder** was born as the result of a desire to learn more about the Rust and WebAssembly environment. The Rust code is compiled all the way down to wasm, so no JS has been used in making this project!

# Live View

You can view it at : [Aww.awarder - Team Praise App](https://nrobert-dev.github.io/aww.arder/)

# Usage

 1. Clone this repository
 2. Installed the required crates
 3. Run the command : *wasm-pack build --target web --out-name wasm --out-dir ./static*
 4. Serve the contents of the /static folder using your favourite file-server

# Under works

The following features are under development/proposed for development:
 1. Save the praises in a database + request the praises when opening the awarder
 2. Praise box live update when a user posts something new

