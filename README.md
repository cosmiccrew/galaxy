`Cosmic Crew: Galaxy`
==================
![CI](https://github.com/cosmiccrew/galaxy/actions/workflows/ci.yml/badge.svg)
![Release](https://github.com/cosmiccrew/galaxy/actions/workflows/release.yml/badge.svg)

Cosmic Crew: Galaxy ~~is~~ will be a 2d, class based gravity oriented fighting game inspired by the likes of Stick Fight: The Game, Super Mario Galaxy and Brawlhalla. **NOTE: This project is in an early stage, and the gameplay or any other related content is subject to change and modification.**

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
4. `cargo run` or (for increased runtime performance, but longer compile times) `cargo run --release`

-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo +nightly fmt`. Note this project uses unstable cargo fmt settings, and requires installing and running cargo fmt on the nighlty edition.

Before submitting a PR or issue, please run the following commands and follow their instructions:
1. `cargo clippy`
2. `cargo +nightly fmt`

#### Dev builds

The development build by default has some **runtime performance** improvements enabled - however, to **speed up compile times** (namely using bevy's internal dynamic linking feature), a simple feature flag can be enabled:
```bash
cargo run --features dynamic_linking
```
You may want to create a `Makefile`, shell alias, or other similar script runner (e.g. [Just](https://just.systems/)) for this.
please note: this will decrease some runtime performance.

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
