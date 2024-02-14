
module {
  func.func private @putchar(i64) -> i64
  func.func private @main() -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c1_i64_0 = arith.constant 1 : i64
    %0 = arith.cmpi eq, %c1_i64, %c1_i64_0 : i64
    scf.if %0 {
      %f = func.constant @putchar : (i64) -> i64
      %c80_i64 = arith.constant 80 : i64
      %c6_i64 = arith.constant 6 : i64
      %1 = arith.addi %c80_i64, %c6_i64 : i64
      %2 = func.call_indirect %f(%1) : (i64) -> i64
    } else {
    }
    %c0_i64 = arith.constant 0 : i64
    return %c0_i64 : i64
  }
}
