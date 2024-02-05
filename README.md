MLIR experiment

## Setup

### macOS

- Install Rust
- Install LLVM 17
- cargo run

### Ubuntu

- Install Rust
- Install LLVM 17 https://apt.llvm.org/
- `apt install libmlir-17-dev libpolly-17-dev libzstd-dev`
- `MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17 cargo run`
