
module {
  func.func private @putchar(i64) -> i64
  func.func private @main() -> i64 {
    %alloca = memref.alloca() : memref<i64>
    %c90_i64 = arith.constant 90 : i64
    memref.store %c90_i64, %alloca[] : memref<i64>
    %0 = memref.load %alloca[] : memref<i64>
    %c91_i64 = arith.constant 91 : i64
    %1 = arith.cmpi eq, %0, %c91_i64 : i64
    scf.if %1 {
      %f = func.constant @putchar : (i64) -> i64
      %c80_i64 = arith.constant 80 : i64
      %c6_i64 = arith.constant 6 : i64
      %2 = arith.addi %c80_i64, %c6_i64 : i64
      %3 = func.call_indirect %f(%2) : (i64) -> i64
    } else {
      %f = func.constant @putchar : (i64) -> i64
      %c80_i64 = arith.constant 80 : i64
      %c6_i64 = arith.constant 6 : i64
      %2 = arith.subi %c80_i64, %c6_i64 : i64
      %3 = func.call_indirect %f(%2) : (i64) -> i64
    }
    %c0_i64 = arith.constant 0 : i64
    return %c0_i64 : i64
  }
}
