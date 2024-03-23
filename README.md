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
  - A Milika function is considered async when it contains a call of an async (Milika or Rust) function.
- main function
  - The entry point of a Milika program must be named `chiika_main` (for now).
- Expressions
  - Integers (1, 2, 3, ...)
  - Variable declaration
    - `alloc x`
    - Only integers are supported now
    - Compiled into llvm's alloca
  - Variable assignment
    - `x = 1`
  - If
    - `if (x) { ... } else { ... }`
  - While
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

- Support `return x` from async Milika func
  - Must be replaced with like `$cont($env, x); return`
- Support `if` in async Milika func
- Support `if` with value 
- Support async call in if condition
- Support `while` in async Milika func

Refactor

- Remove `is_async` from `FunTy`?
  - It is not used after functions are split

## Implementation

### Async + if

(Not implemented yet)

Before:

```
fun foo() -> int {
  if (true) {
    print(1)
    sleep_sec(1)  # Cut point #1
    print(2)
  } else {
    print(3)
    sleep_sec(1)  # Cut point #2
    print(4)
  }
  # Cut point #3 (end of if)
  print(5)
  return 6
}
```

After:

```
fun foo(ChiikaEnv $env, FN((ChiikaEnv,int)->RustFuture) $cont) -> RustFuture {
  chiika_env_push($env, $cont)
  if (true) {
    print(1)
    return sleep_sec($env, foo_1, 1)
  } else {
    print(3)
    return sleep_sec($env, foo_2, 1)
  }
}
fun foo_1(ChiikaEnv $env, Nil $async_result) -> RustFuture {
  print(2)
  return foo_3($env, Nil)
}
fun foo_2(ChiikaEnv $env, Nil $async_result) -> RustFuture {
  print(4)
  return foo_3($env, Nil)
}
fun foo_3(ChiikaEnv $env, Nil $async_result) -> RustFuture {
  print(5)
  return (chiika_env_pop($env, 1))(6)
}

```

### Async + while

(Not implemented yet) `while` can be lowered into a recursive function and then
lowered as usual.

Before:

```
fun foo() -> int {
  print(123)
  alloc i
  i = 0
  # Cut point #1 (beginning of while)
  while (i < 10) {
    i = i + 1
    print(i)
    sleep_sec(1)
    print(i)
  }
  print(789)
  return 0
}
```

After:

```
fun foo(int a) -> int {
  print(a)
  alloc i
  i = 0
  return foo_1(a, i)
}
# Takes original arguments followed by all the alloc'ed variables (for simplicity)
# The return type is the same as the original
fun foo_1(int a, int i_) -> int {
  # Loop termination check
  alloc i
  i = i_
  # Loop termination check
  if (i < 10) {
    # Loop body
    i = i + 1
    print(i)
    sleep_sec(1)
    print(i)
    # Recurse itself
    return foo_1(a, i)
  } else {
    # The part after `while`
    print(789)
    return 0
  }
}
```

## License

MIT
