
module {
  func.func private @print(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
  func.func private @chiika_env_push(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @foo(%arg0: !llvm.ptr, %arg1: i64, %arg2: i64, %arg3: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %0 = builtin.unrealized_conversion_cast %arg3 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %2 = call_indirect %f(%arg0, %1) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %3 = call_indirect %f_0(%arg0, %arg2) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %4 = call_indirect %f_1(%arg0, %arg1) : (!llvm.ptr, i64) -> i64
    %5 = arith.cmpi ult, %arg1, %arg2 : i64
    cf.cond_br %5, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %f_2 = constant @"foo't" : (!llvm.ptr) -> !llvm.ptr
    %6 = call_indirect %f_2(%arg0) : (!llvm.ptr) -> !llvm.ptr
    return %6 : !llvm.ptr
  ^bb2:  // pred: ^bb0
    %f_3 = constant @"foo'f" : (!llvm.ptr) -> !llvm.ptr
    %7 = call_indirect %f_3(%arg0) : (!llvm.ptr) -> !llvm.ptr
    return %7 : !llvm.ptr
  }
  func.func @"foo't"(%arg0: !llvm.ptr) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %f_0 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %0 = call_indirect %f_0(%arg0, %c0_i64) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %1 = call_indirect %f_1(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %2 = arith.addi %0, %1 : i64
    %3 = call_indirect %f(%2) : (i64) -> i64
    %f_2 = constant @sleep_sec : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %c0_i64_3 = arith.constant 0 : i64
    %f_4 = constant @foo_2 : (!llvm.ptr, i64) -> !llvm.ptr
    %4 = call_indirect %f_2(%arg0, %c0_i64_3, %f_4) : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %4 : !llvm.ptr
  }
  func.func @foo_2(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @chiika_env_pop : (!llvm.ptr, i64) -> i64
    %c3_i64 = arith.constant 3 : i64
    %0 = call_indirect %f(%arg0, %c3_i64) : (!llvm.ptr, i64) -> i64
    %1 = llvm.inttoptr %0 : i64 to !llvm.ptr
    %2 = builtin.unrealized_conversion_cast %1 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %f_0 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %3 = call_indirect %f_0(%arg0, %c0_i64) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %4 = call_indirect %f_1(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %5 = arith.addi %3, %4 : i64
    %6 = call_indirect %2(%arg0, %5) : (!llvm.ptr, i64) -> !llvm.ptr
    return %6 : !llvm.ptr
  }
  func.func @"foo'f"(%arg0: !llvm.ptr) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %f_0 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %0 = call_indirect %f_0(%arg0, %c0_i64) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %1 = call_indirect %f_1(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %2 = arith.addi %0, %1 : i64
    %3 = call_indirect %f(%2) : (i64) -> i64
    %f_2 = constant @sleep_sec : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %c0_i64_3 = arith.constant 0 : i64
    %f_4 = constant @foo_4 : (!llvm.ptr, i64) -> !llvm.ptr
    %4 = call_indirect %f_2(%arg0, %c0_i64_3, %f_4) : (!llvm.ptr, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %4 : !llvm.ptr
  }
  func.func @foo_4(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @chiika_env_pop : (!llvm.ptr, i64) -> i64
    %c3_i64 = arith.constant 3 : i64
    %0 = call_indirect %f(%arg0, %c3_i64) : (!llvm.ptr, i64) -> i64
    %1 = llvm.inttoptr %0 : i64 to !llvm.ptr
    %2 = builtin.unrealized_conversion_cast %1 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %f_0 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %3 = call_indirect %f_0(%arg0, %c0_i64) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %4 = call_indirect %f_1(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %5 = arith.addi %3, %4 : i64
    %6 = call_indirect %2(%arg0, %5) : (!llvm.ptr, i64) -> !llvm.ptr
    return %6 : !llvm.ptr
  }
  func.func @chiika_main(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %0 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %2 = call_indirect %f(%arg0, %1) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @foo : (!llvm.ptr, i64, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %f_1 = constant @chiika_main_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %3 = call_indirect %f_0(%arg0, %c1_i64, %c2_i64, %f_1) : (!llvm.ptr, i64, i64, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    return %3 : !llvm.ptr
  }
  func.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %0 = call_indirect %f(%arg1) : (i64) -> i64
    %f_0 = constant @chiika_env_pop : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %1 = call_indirect %f_0(%arg0, %c1_i64) : (!llvm.ptr, i64) -> i64
    %2 = llvm.inttoptr %1 : i64 to !llvm.ptr
    %3 = builtin.unrealized_conversion_cast %2 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %4 = call_indirect %3(%arg0, %c0_i64) : (!llvm.ptr, i64) -> !llvm.ptr
    return %4 : !llvm.ptr
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
