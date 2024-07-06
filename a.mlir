
module {
  func.func private @print(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
  func.func private @chiika_env_push_frame(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_set(!llvm.ptr, i64, i64, i64) -> i64
  func.func private @chiika_env_pop_frame(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_ref(!llvm.ptr, i64, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @chiika_main(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push_frame : (!llvm.ptr, i64) -> i64
    %c2_i64 = arith.constant 2 : i64
    %0 = call_indirect %f(%arg0, %c2_i64) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @chiika_env_set : (!llvm.ptr, i64, i64, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %1 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %2 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %c6_i64 = arith.constant 6 : i64
    %3 = call_indirect %f_0(%arg0, %c0_i64, %2, %c6_i64) : (!llvm.ptr, i64, i64, i64) -> i64
    %f_1 = constant @chiika_env_set : (!llvm.ptr, i64, i64, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c1_i64_2 = arith.constant 1 : i64
    %4 = call_indirect %f_1(%arg0, %c1_i64, %c3_i64, %c1_i64_2) : (!llvm.ptr, i64, i64, i64) -> i64
    %f_3 = constant @print : (i64) -> i64
    %f_4 = constant @chiika_env_ref : (!llvm.ptr, i64, i64) -> i64
    %c1_i64_5 = arith.constant 1 : i64
    %c1_i64_6 = arith.constant 1 : i64
    %5 = call_indirect %f_4(%arg0, %c1_i64_5, %c1_i64_6) : (!llvm.ptr, i64, i64) -> i64
    %6 = call_indirect %f_3(%5) : (i64) -> i64
    %f_7 = constant @sleep_sec : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %c1_i64_8 = arith.constant 1 : i64
    %f_9 = constant @chiika_main_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %7 = call_indirect %f_7(%arg0, %c1_i64_8, %f_9) : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %7 : !llvm.ptr
  }
  func.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %f_0 = constant @chiika_env_ref : (!llvm.ptr, i64, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %c1_i64_1 = arith.constant 1 : i64
    %0 = call_indirect %f_0(%arg0, %c1_i64, %c1_i64_1) : (!llvm.ptr, i64, i64) -> i64
    %1 = call_indirect %f(%0) : (i64) -> i64
    %alloca = memref.alloca() : memref<i64>
    %c0_i64 = arith.constant 0 : i64
    memref.store %c0_i64, %alloca[] : memref<i64>
    %f_2 = constant @chiika_env_pop_frame : (!llvm.ptr, i64) -> i64
    %c2_i64 = arith.constant 2 : i64
    %2 = call_indirect %f_2(%arg0, %c2_i64) : (!llvm.ptr, i64) -> i64
    %3 = llvm.inttoptr %2 : i64 to !llvm.ptr
    %4 = builtin.unrealized_conversion_cast %3 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %5 = memref.load %alloca[] : memref<i64>
    %6 = call_indirect %4(%arg0, %5) : (!llvm.ptr, i64) -> !llvm.ptr
    return %6 : !llvm.ptr
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
