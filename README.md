`Cosmic Crew: Galaxy`
==================
![CI](https://github.com/cosmiccrew/galaxy/actions/workflows/ci.yml/badge.svg)
[![Pages](https://github.com/cosmiccrew/galaxy/actions/workflows/pages.yml/badge.svg)](https://cosmiccrew.github.io/galaxy)
![Release](https://github.com/cosmiccrew/galaxy/actions/workflows/release.yml/badge.svg)

Cosmic Crew: Galaxy ~~is~~ will be a 2d, class based gravity oriented fighting game inspired by the likes of Stick Fight: The Game, Super Mario Galaxy and Brawlhalla. **NOTE: This project is in an early stage, and the gameplay or any other related content is subject to change, modification and overhaul.**

-------

## Download & play

To run and play Cosmic Crew: Galaxy, there are a few options:

#### Releases

1. By downloading either a specifc release or 'nightly version of the game from the github [releases page](https://github.com/cosmiccrew/galaxy/releases)
2. Extracting the archive (if necessary)
3. Running or opening the executable


#### From source

1. Install rust at [rustup.rs](https://rustup.rs)
2. Clone the repo `git clone https://github.com/cosmiccrew/galaxy.git`
3. `cd galaxy`
4. `cargo run` (faster compiles, slower performance) or `cargo run --release` (increased runtime performance, but longer compile times)

#### Using WASM
1. follow the from source instructions, skipping step 4
2. `cargo run --profile wasm --target wasm32-unknown-unknown`, running with bevy's webgpu backend
NOTE: (this uses `RUSTFLAGS="--cfg=web_sys_unstable_apis"` and `wasm-server-runner` to enable webgpu support - take a look at the wasm section of `.cargo/recommended-config.toml` for details)
-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo +nightly fmt`. Note this project uses unstable cargo fmt settings, and requires installing and running cargo fmt on the nightly edition.

Before submitting a PR or issue, please run the following commands and follow their instructions:
1. `cargo clippy`
2. `cargo +nightly fmt` (requires install the "nightly" toolchain via `rustup install nightly`)

#### Dev builds

The development build by default has some **runtime performance** improvements enabled - however, to **speed up compile times** (namely using bevy's internal dynamic linking feature), a simple feature flag can be enabled:
```bash
cargo run --features fast_compile
```
NOTE: this will decrease runtime performance somewhat.

there is also option debug features and plugins to help with development that are enabled with the "debug" feature:
```bash
cargo run --features debug
```

You may want to create a cargo alias (like those found in `.cargo/recommended-config.toml`) to have a shorthand for these commands.

-------

## Credits

This project wouldn't be possible without the hard work and dedication put in by these various projects:
* [bevy](https://github.com/bevyengine/bevy/) - the game engine galaxy is built with, with an incredibly extensible design and countless helpful features
* [FishFolk: Jumpy](https://github.com/fishfolk/jumpy) - a 2d couch-shooter game built in bevy, serving as great inspiration that building a full game in bevy is possible
* [Deep-Fold's Pixel Planets](https://deep-fold.itch.io/pixel-planet-generator) - a set of shaders that create amazing 2d pixelated planets, of which shader's are extensively used in this game's planets

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
