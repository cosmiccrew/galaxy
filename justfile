run: fmt
  cargo run --features=dynamic_linking

build: fmt
  cargo build --features=dynamic_linking

fmt:
  cargo +nightly fmt

check:
  cargo check

test:
  cargo test --features=dynamic_linking

actions: fmt check test
