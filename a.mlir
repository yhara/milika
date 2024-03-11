
"builtin.module"() ({
  "func.func"() <{function_type = (i64) -> i64, sym_name = "putchar", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (i64) -> i64, sym_name = "sleep_sec", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, !llvm.ptr) -> i64, sym_name = "chiika_env_push", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "chiika_env_pop", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> i64, sym_name = "chiika_env_ref", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (i64) -> i64, sym_name = "chiika_start_tokio", sym_visibility = "private"}> ({
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, sym_name = "chiika_main"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr):
    %0 = "func.constant"() <{value = @chiika_env_push}> : () -> ((!llvm.ptr, !llvm.ptr) -> i64)
    %1 = "llvm.call"(%0, %arg0, %arg1) <{fastmathFlags = #llvm.fastmath<none>}> : ((!llvm.ptr, !llvm.ptr) -> i64, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> i64
    %2 = "func.constant"() <{value = @putchar}> : () -> ((i64) -> i64)
    %3 = "arith.constant"() <{value = 70 : i64}> : () -> i64
    %4 = "llvm.call"(%2, %3) <{fastmathFlags = #llvm.fastmath<none>}> : ((i64) -> i64, i64) -> i64
    %5 = "func.constant"() <{value = @sleep_sec}> : () -> ((i64) -> i64)
    %6 = "func.constant"() <{value = @chiika_main_1}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %7 = "arith.constant"() <{value = 1 : i64}> : () -> i64
    %8 = "llvm.call"(%5, %arg0, %6, %7) <{fastmathFlags = #llvm.fastmath<none>}> : ((i64) -> i64, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> i64
    "func.return"(%8) : (i64) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "chiika_main_1"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: i64):
    %0 = "func.constant"() <{value = @putchar}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 72 : i64}> : () -> i64
    %2 = "llvm.call"(%0, %1) <{fastmathFlags = #llvm.fastmath<none>}> : ((i64) -> i64, i64) -> i64
    %3 = "func.constant"() <{value = @chiika_env_pop}> : () -> ((!llvm.ptr, i64) -> ((!llvm.ptr, i64) -> !llvm.ptr))
    %4 = "arith.constant"() <{value = 1 : i64}> : () -> i64
    %5 = "llvm.call"(%3, %arg0, %4) <{fastmathFlags = #llvm.fastmath<none>}> : ((!llvm.ptr, i64) -> ((!llvm.ptr, i64) -> !llvm.ptr), !llvm.ptr, i64) -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %6 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %7 = "llvm.call"(%5, %arg0, %6) <{fastmathFlags = #llvm.fastmath<none>}> : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, sym_name = "chiika_start_user"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr):
    %0 = "func.constant"() <{value = @chiika_main}> : () -> ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr)
    %1 = "llvm.call"(%0, %arg0, %arg1) <{fastmathFlags = #llvm.fastmath<none>}> : ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    "func.return"(%1) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "main"}> ({
    %0 = "func.constant"() <{value = @chiika_start_tokio}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %2 = "llvm.call"(%0, %1) <{fastmathFlags = #llvm.fastmath<none>}> : ((i64) -> i64, i64) -> i64
    %3 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    "func.return"(%3) : (i64) -> ()
  }) : () -> ()
}) : () -> ()
