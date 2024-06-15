
module {
  func.func private @print(i64) -> i64
  func.func private @chiika_env_push(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @chiika_main() -> i64 {
    %true = arith.constant true
    cf.cond_br %true, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %f = constant @print : (i64) -> i64
    %c456_i64 = arith.constant 456 : i64
    %0 = call_indirect %f(%c456_i64) : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    cf.br ^bb3(%c0_i64 : i64)
  ^bb2:  // pred: ^bb0
    %f_0 = constant @print : (i64) -> i64
    %c789_i64 = arith.constant 789 : i64
    %1 = call_indirect %f_0(%c789_i64) : (i64) -> i64
    %c0_i64_1 = arith.constant 0 : i64
    cf.br ^bb3(%c0_i64_1 : i64)
  ^bb3(%2: i64):  // 2 preds: ^bb1, ^bb2
    %c0_i64_2 = arith.constant 0 : i64
    return %c0_i64_2 : i64
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
