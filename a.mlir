
module {
  func.func private @putchar(i64) -> i64
  func.func private @main() -> i64 {
    %token, %bodyResults = async.execute -> !async.value<i64> {
      %f_2 = func.constant @putchar : (i64) -> i64
      %c65_i64 = arith.constant 65 : i64
      %3 = func.call_indirect %f_2(%c65_i64) : (i64) -> i64
      %f_3 = func.constant @putchar : (i64) -> i64
      %c66_i64 = arith.constant 66 : i64
      %4 = func.call_indirect %f_3(%c66_i64) : (i64) -> i64
      %f_4 = func.constant @putchar : (i64) -> i64
      %c67_i64 = arith.constant 67 : i64
      %5 = func.call_indirect %f_4(%c67_i64) : (i64) -> i64
      %c0_i64_5 = arith.constant 0 : i64
      async.yield %c0_i64_5 : i64
    }
    %f = constant @putchar : (i64) -> i64
    %c68_i64 = arith.constant 68 : i64
    %0 = call_indirect %f(%c68_i64) : (i64) -> i64
    %f_0 = constant @putchar : (i64) -> i64
    %c69_i64 = arith.constant 69 : i64
    %1 = call_indirect %f_0(%c69_i64) : (i64) -> i64
    %f_1 = constant @putchar : (i64) -> i64
    %c70_i64 = arith.constant 70 : i64
    %2 = call_indirect %f_1(%c70_i64) : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    return %c0_i64 : i64
  }
}
