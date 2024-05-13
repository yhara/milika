
// -- typing output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(Int n) -> Null;
// fun countup[?](Int i) -> Null {
//   print[+](%arg_0)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   return countup[?]((%arg_0 + 1))  # Null  #-> Void
// }
// fun chiika_main[?]() -> Null {
//   countup[?](0)  #-> Null
//   return null  # Null  #-> Void
// }
// 
// -- lower_async_if output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(Int n) -> Null;
// fun countup[?](Int i) -> Null {
//   print[+](%arg_0)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   return countup[?]((%arg_0 + 1))  # Null  #-> Void
// }
// fun chiika_main[?]() -> Null {
//   countup[?](0)  #-> Null
//   return null  # Null  #-> Void
// }
// 
// -- asyncness_check output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(Int n) -> Null;
// fun countup[*](Int i) -> Null {
//   print[+](%arg_0)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   return countup[*]((%arg_0 + 1))  # Null  #-> Void
// }
// fun chiika_main[*]() -> Null {
//   countup[*](0)  #-> Null
//   return null  # Null  #-> Void
// }
// 
// -- async_splitter output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int n) -> RustFuture;
// fun countup[.](ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int i) -> RustFuture {
//   chiika_env_push[.](%arg_0, FunToAny(%arg_1))  #-> Int
//   chiika_env_push[.](%arg_0, IntToAny(%arg_2))  #-> Int
//   print[+](%arg_0)  #-> Null
//   return sleep_sec[.](%arg_0, countup_1, 1)  # RustFuture  #-> Void
// }
// fun countup_1[.](ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1  #-> Null
//   return countup[.](%arg_0, countup_2, (chiika_env_ref[.](%arg_0, 0) + 1))  # RustFuture  #-> Void
// }
// fun countup_2[.](ChiikaEnv $env, Null $async_result) -> RustFuture {
//   return AnyToFun((ChiikaEnv,Null)->RustFuture)(chiika_env_pop[.](%arg_0, 2))[.](%arg_0, %arg_1)  # RustFuture  #-> Void
// }
// fun chiika_main[.](ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont) -> RustFuture {
//   chiika_env_push[.](%arg_0, FunToAny(%arg_1))  #-> Int
//   return countup[.](%arg_0, chiika_main_1, 0)  # RustFuture  #-> Void
// }
// fun chiika_main_1[.](ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1  #-> Null
//   return AnyToFun((ChiikaEnv,Null)->RustFuture)(chiika_env_pop[.](%arg_0, 1))[.](%arg_0, null)  # RustFuture  #-> Void
// }
// 
// -- verifier input --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int n) -> RustFuture;
// extern chiika_env_push(ChiikaEnv env, Any obj) -> Null;
// extern chiika_env_pop(ChiikaEnv env, Int n) -> Any;
// extern chiika_env_ref(ChiikaEnv env, Int n) -> Int;
// extern chiika_start_tokio(Int n) -> Int;
// fun countup(ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int i) -> RustFuture {
//   chiika_env_push[.](%arg_0, FunToAny(%arg_1));  #-> Int
//   chiika_env_push[.](%arg_0, IntToAny(%arg_2));  #-> Int
//   print[+](%arg_0);  #-> Null
//   return sleep_sec[.](%arg_0, countup_1, 1)  # RustFuture;  #-> Void
// }
// fun countup_1(ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1;  #-> Null
//   return countup[.](%arg_0, countup_2, (chiika_env_ref[.](%arg_0, 0) + 1))  # RustFuture;  #-> Void
// }
// fun countup_2(ChiikaEnv $env, Null $async_result) -> RustFuture {
//   return AnyToFun((ChiikaEnv,Null)->RustFuture)(chiika_env_pop[.](%arg_0, 2))[.](%arg_0, %arg_1)  # RustFuture;  #-> Void
// }
// fun chiika_main(ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont) -> RustFuture {
//   chiika_env_push[.](%arg_0, FunToAny(%arg_1));  #-> Int
//   return countup[.](%arg_0, chiika_main_1, 0)  # RustFuture;  #-> Void
// }
// fun chiika_main_1(ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1;  #-> Null
//   return AnyToFun((ChiikaEnv,Null)->RustFuture)(chiika_env_pop[.](%arg_0, 1))[.](%arg_0, null)  # RustFuture;  #-> Void
// }
// fun chiika_start_user(ChiikaEnv env, (ChiikaEnv,Int)->RustFuture cont) -> RustFuture {
//   return chiika_main[+](%arg_0, %arg_1)  # RustFuture;  #-> Void
// }
// fun main() -> Int {
//   chiika_start_tokio[+](0);  #-> Int
//   return 0  # Int;  #-> Void
// }
// 
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
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr, sym_name = "countup"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr, %arg2: i64):
    %0 = "func.constant"() <{value = @chiika_env_push}> : () -> ((!llvm.ptr, !llvm.ptr) -> i64)
    %1 = "builtin.unrealized_conversion_cast"(%arg1) : ((!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %2 = "func.call_indirect"(%0, %arg0, %1) : ((!llvm.ptr, !llvm.ptr) -> i64, !llvm.ptr, !llvm.ptr) -> i64
    %3 = "func.constant"() <{value = @chiika_env_push}> : () -> ((!llvm.ptr, !llvm.ptr) -> i64)
    %4 = "llvm.inttoptr"(%arg2) : (i64) -> i64
    %5 = "func.call_indirect"(%3, %arg0, %4) : ((!llvm.ptr, !llvm.ptr) -> i64, !llvm.ptr, i64) -> i64
    %6 = "func.constant"() <{value = @print}> : () -> ((i64) -> i64)
    %7 = "func.call_indirect"(%6, %arg0) : ((i64) -> i64, !llvm.ptr) -> i64
    %8 = "func.constant"() <{value = @sleep_sec}> : () -> ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr)
    %9 = "func.constant"() <{value = @countup_1}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %10 = "arith.constant"() <{value = 1 : i64}> : () -> i64
    %11 = "func.call_indirect"(%8, %arg0, %9, %10) : ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%11) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "countup_1"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: i64):
    %0 = "func.constant"() <{value = @countup}> : () -> ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr)
    %1 = "func.constant"() <{value = @countup_2}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %2 = "func.constant"() <{value = @chiika_env_ref}> : () -> ((!llvm.ptr, i64) -> i64)
    %3 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %4 = "func.call_indirect"(%2, %arg0, %3) : ((!llvm.ptr, i64) -> i64, !llvm.ptr, i64) -> i64
    %5 = "arith.constant"() <{value = 1 : i64}> : () -> i64
    %6 = "arith.addi"(%4, %5) : (i64, i64) -> i64
    %7 = "func.call_indirect"(%0, %arg0, %1, %6) : ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%7) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "countup_2"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: i64):
    %0 = "func.constant"() <{value = @chiika_env_pop}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %1 = "arith.constant"() <{value = 2 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %arg0, %1) : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    %3 = "builtin.unrealized_conversion_cast"(%2) : (!llvm.ptr) -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %4 = "func.call_indirect"(%3, %arg0, %arg1) : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%4) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, sym_name = "chiika_main"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr):
    %0 = "func.constant"() <{value = @chiika_env_push}> : () -> ((!llvm.ptr, !llvm.ptr) -> i64)
    %1 = "builtin.unrealized_conversion_cast"(%arg1) : ((!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    %2 = "func.call_indirect"(%0, %arg0, %1) : ((!llvm.ptr, !llvm.ptr) -> i64, !llvm.ptr, !llvm.ptr) -> i64
    %3 = "func.constant"() <{value = @countup}> : () -> ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr)
    %4 = "func.constant"() <{value = @chiika_main_1}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %5 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %6 = "func.call_indirect"(%3, %arg0, %4, %5) : ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%6) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, i64) -> !llvm.ptr, sym_name = "chiika_main_1"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: i64):
    %0 = "func.constant"() <{value = @chiika_env_pop}> : () -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %1 = "arith.constant"() <{value = 1 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %arg0, %1) : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    %3 = "builtin.unrealized_conversion_cast"(%2) : (!llvm.ptr) -> ((!llvm.ptr, i64) -> !llvm.ptr)
    %4 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %5 = "func.call_indirect"(%3, %arg0, %4) : ((!llvm.ptr, i64) -> !llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    "func.return"(%5) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, sym_name = "chiika_start_user"}> ({
  ^bb0(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr):
    %0 = "func.constant"() <{value = @chiika_main}> : () -> ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr)
    %1 = "func.call_indirect"(%0, %arg0, %arg1) : ((!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr, !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr
    "func.return"(%1) : (!llvm.ptr) -> ()
  }) : () -> ()
  "func.func"() <{function_type = () -> i64, sym_name = "main"}> ({
    %0 = "func.constant"() <{value = @chiika_start_tokio}> : () -> ((i64) -> i64)
    %1 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    %2 = "func.call_indirect"(%0, %1) : ((i64) -> i64, i64) -> i64
    %3 = "arith.constant"() <{value = 0 : i64}> : () -> i64
    "func.return"(%3) : (i64) -> ()
  }) : () -> ()
}) : () -> ()
