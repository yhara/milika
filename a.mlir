
module {
  func.func private @putchar(i64) -> i64
  func.func private @main() -> i64 {
    %f = constant @putchar : (i64) -> i64
    %c80_i64 = arith.constant 80 : i64
    %c6_i64 = arith.constant 6 : i64
    %0 = arith.addi %c80_i64, %c6_i64 : i64
    %1 = call_indirect %f(%0) : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    return %c0_i64 : i64
  }
}
