
module {
  func.func private @putchar(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
  func.func private @chiika_env_push(!llvm.ptr, !llvm.ptr) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> !llvm.ptr
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @chiika_main(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, !llvm.ptr) -> i64
    %0 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = call_indirect %f(%arg0, %0) : (!llvm.ptr, !llvm.ptr) -> i64
    %f_0 = constant @putchar : (i64) -> i64
    %c70_i64 = arith.constant 70 : i64
    %2 = call_indirect %f_0(%c70_i64) : (i64) -> i64
    %f_1 = constant @sleep_sec : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    %f_2 = constant @chiika_main_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %3 = call_indirect %f_1(%arg0, %f_2, %c1_i64) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    return %3 : !llvm.ptr
  }
  func.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @putchar : (i64) -> i64
    %c72_i64 = arith.constant 72 : i64
    %0 = call_indirect %f(%c72_i64) : (i64) -> i64
    %f_0 = constant @chiika_env_pop : (!llvm.ptr, i64) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %1 = call_indirect %f_0(%arg0, %c1_i64) : (!llvm.ptr, i64) -> !llvm.ptr
    %2 = builtin.unrealized_conversion_cast %1 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %3 = call_indirect %2(%arg0, %c0_i64) : (!llvm.ptr, i64) -> !llvm.ptr
    return %3 : !llvm.ptr
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
