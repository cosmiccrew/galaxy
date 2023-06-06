run: fmt
  cargo run --features=dynamic_linking

build: fmt
  cargo build --features=dynamic_linking

#correctness/testing

fmt:
  cargo +nightly fmt

clippy:
  cargo clippy --all-targets --features=dynamic_linking -- -D warnings

test:
  cargo test --all --features=dynamic_linking

actions: fmt clippy test
