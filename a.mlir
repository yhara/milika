
module {
  func.func private @print(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
  func.func private @chiika_env_push(!llvm.ptr, !llvm.ptr) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> !llvm.ptr
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @chiika_main() -> i64 {
    %true = arith.constant true
    %0 = scf.if %true -> (i64) {
      %f = func.constant @"chiika_main't" : () -> i64
      %1 = func.call_indirect %f() : () -> i64
      scf.yield %1 : i64
    } else {
      %f = func.constant @"chiika_main'f" : () -> i64
      %1 = func.call_indirect %f() : () -> i64
      scf.yield %1 : i64
    }
    return %0 : i64
  }
  func.func @"chiika_main't"() -> i64 {
    %f = constant @"chiika_main'e" : (i64) -> i64
    %c456_i64 = arith.constant 456 : i64
    %0 = call_indirect %f(%c456_i64) : (i64) -> i64
    return %0 : i64
  }
  func.func @"chiika_main'f"() -> i64 {
    %f = constant @"chiika_main'e" : (i64) -> i64
    %c789_i64 = arith.constant 789 : i64
    %0 = call_indirect %f(%c789_i64) : (i64) -> i64
    return %0 : i64
  }
  func.func @"chiika_main'e"(%arg0: i64) -> i64 {
    %f = constant @print : (i64) -> i64
    %0 = call_indirect %f(%arg0) : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    return %c0_i64 : i64
  }
  func.func @chiika_start_user(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_main : () -> i64
    %0 = call_indirect %f() : () -> i64
    %1 = call_indirect %arg1(%arg0, %0) : (!llvm.ptr, i64) -> !llvm.ptr
    return %1 : !llvm.ptr
  }
  func.func @main() -> i64 {
    %f = constant @chiika_start_tokio : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %0 = call_indirect %f(%c0_i64) : (i64) -> i64
    %c0_i64_0 = arith.constant 0 : i64
    return %c0_i64_0 : i64
  }
}
