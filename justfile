work:
    cargo watch -x "check" -x "clippy" -x "build" -s "just test" --ignore "dist"
build:
    cargo check; cargo build; just clippy; cargo test
clippy:
    cargo clippy
test:
    cargo nextest run
debug:
    cargo run --features debug -- -l Debug
# bench:
#     cargo bench -q > benchmarks.txt
flamegraph:
    cargo flamegraph --profile flamegraph --root --bin galaxy
dhat:
    cargo run --profile dhat --features dhat-heap
run:
    cargo run
fmt:
    cargo +nightly fmt; taplo fmt
prepare: fmt clippy test
