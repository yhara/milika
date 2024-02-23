
module {
  func.func private @putchar(i64) -> i64
  func.func private @main() -> i64 {
    %alloca = memref.alloca() : memref<i64>
    %c0_i64 = arith.constant 0 : i64
    memref.store %c0_i64, %alloca[] : memref<i64>
    %0 = memref.load %alloca[] : memref<i64>
    %c3_i64 = arith.constant 3 : i64
    %1 = arith.cmpi ult, %0, %c3_i64 : i64
    scf.while : () -> () {
      %2 = memref.load %alloca[] : memref<i64>
      %c3_i64_1 = arith.constant 3 : i64
      %3 = arith.cmpi ult, %2, %c3_i64_1 : i64
      scf.condition(%3)
    } do {
      %f = func.constant @putchar : (i64) -> i64
      %c80_i64 = arith.constant 80 : i64
      %2 = memref.load %alloca[] : memref<i64>
      %3 = arith.addi %c80_i64, %2 : i64
      %4 = func.call_indirect %f(%3) : (i64) -> i64
      %5 = memref.load %alloca[] : memref<i64>
      %c1_i64 = arith.constant 1 : i64
      %6 = arith.addi %5, %c1_i64 : i64
      memref.store %6, %alloca[] : memref<i64>
      scf.yield
    }
    %c0_i64_0 = arith.constant 0 : i64
    return %c0_i64_0 : i64
  }
}
