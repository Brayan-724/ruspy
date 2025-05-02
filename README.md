# Useful commands

All are available with `./build` (needs nushell)

### ./build test \[$test\]
```bash
cargo test --no-fail-fast --lib --verbose $test -- --nocapture
```

### ./build run $path
```bash
cargo run --bin main -- $path
```

### ./build lexer $path
```bash
cargo run --bin inspect-lexer -- $path
```

### ./build ast $path
```bash
cargo run --bin inspect-ast -- $path
```

### ./build inspect $path
```bash
cargo run --bin inspect -- $path
```

### ./build to-js $path
```bash
cargo run --bin to-js -- $path
```
