# Area 25.5

Area 25.5 is about an Alien stuck on Earth trying to find his other half and then leave this dark planet out to his own Universe.

This is part of the submission to the Pirate Software's Game Jam 15. You can play it at [itch.io](https://guilospanck.itch.io/area25-5).

Obs.: in development.

## Tech

Built with rust and bevy.

## Installation

Be sure to have `rust` and the `bevy` [required dependencies based on the OS](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies) installed.
Also, to run `just` commands, [install it](https://github.com/casey/just?tab=readme-ov-file#packages) (or just open the `.justfile` and run the corresponding commands manually).

## Running locally

`just run` and wait for it to build and compile.

## Building for the web

### Requirements

- Before doing it, make sure to have [`binaryen`](https://github.com/WebAssembly/binaryen/releases) installed and in the correct path (change the `.justfile` "optimise-web" if needed);
- Also install the wasm32-unknown-unknown rustup target (`rustup target add wasm32-unknown-unknown`);
- And [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) is needed.

- Go to `cargo.toml` and follow the instructions there concerning the `bevy-inspector-egui` (you should comment out the "bevy-inspector-egui" line that has the "optional = true", uncomment the one that does not have optional and finally comment out the "not_web" line under `[features]`);
- Go to `main.rs` and follow the assets instructions to create the `meta` files (Optional, only if you added new assets);
- Run `just web`
