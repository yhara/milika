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

See examples/\*.milika or src/ast.rs.

- Asyncness
  - Async Rust function must be imported with `extern(async)`.
  - There is no "async mark" for milika functions. 
- main function
  - The entry point of a Milika program must be named `chiika_main` (for now).
- Expressions
  - Integers (1, 2, 3, ...)
  - Variable declaration
    - `alloc x`
    - Only integers are supported now
  - Variable assignment
    - `x = 1`
  - If
    - `if (x) { ... } else { ... }` #=> statement if
    - `if (x) { ... yield 1 } else { ... yield 2 }` #=> expression if
  - While (TODO)
    - `while (x) { ... }`
  - Return
    - `return x`
- Types
  - `Int`
  - `Bool`
  - `Null`
    - A type that can only have one value, `null`.
  - Internally used in src/prelude.rs:
    - `ENV` (chiika_runtime/src/chiika_env.rs)
    - `ANY` (llvm ptr)
    - `FN((x,y)->z)` (function type)

## TODO

- Support `while` in async Milika func

Refactoring

- Rename `Void` to `Never`

## License

MIT
