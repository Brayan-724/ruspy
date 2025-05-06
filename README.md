# Useful commands

All are available with `./build` (needs nushell)

### ./build --help
Show the list of commands

### ./build test \[$test\]
```bash
cargo test --no-fail-fast --lib --verbose $test -- --nocapture
```

### ./build run $path
```bash
cargo run --bin main -- $path
```
