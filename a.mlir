
"builtin.module"() ({
  "func.func"() <{function_type = (i64) -> i64, sym_name = "print", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr, sym_name = "sleep_sec", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, !llvm.ptr) -> i64, sym_name = "chiika_env_push", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "chiika_env_pop", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> i64, sym_name = "chiika_env_ref", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (i64) -> i64, sym_name = "chiika_start_tokio", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "chiika_main"}> ({
    %0 = "func.constant"() <{value = @print}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 123 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %1) : ((i64) -> i64, i64) -> i64
    %3 = "arith.constant"() <{value = true}> : () -> i1
    "scf.if"(%3) ({
      %4 = "func.constant"() <{value = @"chiika_main't"}> : () -> (() -> i64)
      %5 = "func.call_indirect"(%4) : (() -> i64) -> i64
      "func.return"(%5) : (i64) -> ()
    }, {
      %4 = "func.constant"() <{value = @"chiika_main'f"}> : () -> (() -> i64)
      %5 = "func.call_indirect"(%4) : (() -> i64) -> i64
      "func.return"(%5) : (i64) -> ()
    }) : (i1) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "chiika_main't"}> ({
    %0 = "func.constant"() <{value = @print}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 456 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %1) : ((i64) -> i64, i64) -> i64
    %3 = "func.constant"() <{value = @"chiika_main'e"}> : () -> (() -> i64)
    %4 = "func.call_indirect"(%3) : (() -> i64) -> i64
    "func.return"(%4) : (i64) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "chiika_main'f"}> ({
    %0 = "func.constant"() <{value = @print}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 789 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %1) : ((i64) -> i64, i64) -> i64
    %3 = "func.constant"() <{value = @"chiika_main'e"}> : () -> (() -> i64)
    %4 = "func.call_indirect"(%3) : (() -> i64) -> i64
    "func.return"(%4) : (i64) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "chiika_main'e"}> ({
    %0 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %1 = "func.constant"() <{value = @print}> : () -> ((i64) -> i64)
    %2 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %3 = "func.call_indirect"(%1, %2) : ((i64) -> i64, i64) -> i64
    %4 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    "func.return"(%4) : (i64) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, sym_name = "chiika_start_user"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr):
    %0 = "func.constant"() <{value = @chiika_main}> : () -> (() -> i64)
    %1 = "func.call_indirect"(%0) : (() -> i64) -> i64
    %2 = "func.call_indirect"(%arg1, %arg0, %1) : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%2) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "main"}> ({
    %0 = "func.constant"() <{value = @chiika_start_tokio}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %1) : ((i64) -> i64, i64) -> i64
    %3 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    "func.return"(%3) : (i64) -> ()
  }) : () -> ()
}) : () -> ()
