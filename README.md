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

## How to run

Currently you need Ruby and Rake (`gem install rake`) to run a milika program.

Example:

```sh
$ NAME="examples/sync" rake run
...
123
```

## Language

See a.milika or src/ast.rs.

- Asyncness
  - Async Rust function must be imported with `extern(async)`.
  - A Milika function is considered async when it contains a call of an async (Milika or Rust) function.

## TODO

- Support `return x` from async Milika func
  - Must be replaced with like `$cont($env, x); return;`
- Support `if` in async Milika func
- Support `while` in async Milika func

Refactor

- Remove `is_async` from `FunTy`?
  - It is not used after functions are split
