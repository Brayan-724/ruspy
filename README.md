# Useful commands

All are available with `./build` (needs nushell)

### ./build test 
```bash
cargo test --no-fail-fast --lib
```

### ./build lexer $path
```bash
cargo run --bin tokens-inspect -- $path
```

### ./build run $path
```bash
cargo run --bin main -- $path
```

### ./build ast $path
```bash
cargo run --bin ast-inspect -- $path
```

### ./build to-js $path
```bash
cargo run --bin to-js -- $path
```

### ./build inspect $path
```bash
cargo run --bin inspect -- $path
```
