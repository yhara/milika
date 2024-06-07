
module {
  func.func private @print(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
  func.func private @chiika_env_push(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @chiika_main(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %0 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %2 = call_indirect %f(%arg0, %1) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @sleep_sec : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %f_1 = constant @chiika_main_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %3 = call_indirect %f_0(%arg0, %c1_i64, %f_1) : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %3 : !llvm.ptr
  }
  func.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %true = arith.constant true
    cf.cond_br %true, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %f = constant @print : (i64) -> i64
    %c123_i64 = arith.constant 123 : i64
    %0 = call_indirect %f(%c123_i64) : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    cf.br ^bb3(%c0_i64 : i64)
  ^bb2:  // pred: ^bb0
    %c0_i64_0 = arith.constant 0 : i64
    cf.br ^bb3(%c0_i64_0 : i64)
  ^bb3(%1: i64):  // 2 preds: ^bb1, ^bb2
    %f_1 = constant @chiika_env_pop : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %2 = call_indirect %f_1(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %3 = llvm.inttoptr %2 : i64 to !llvm.ptr
    %4 = builtin.unrealized_conversion_cast %3 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %c0_i64_2 = arith.constant 0 : i64
    %5 = call_indirect %4(%arg0, %c0_i64_2) : (!llvm.ptr, i64) -> !llvm.ptr
    return %5 : !llvm.ptr
  }
  func.func @chiika_start_user(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_main : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %0 = call_indirect %f(%arg0, %arg1) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %0 : !llvm.ptr
  }
  func.func @main() -> i64 {
    %f = constant @chiika_start_tokio : (i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %0 = call_indirect %f(%c0_i64) : (i64) -> i64
    %c0_i64_0 = arith.constant 0 : i64
    return %c0_i64_0 : i64
  }
}
