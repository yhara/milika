
// -- typing output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(Int n) -> Null;
// fun countup[?](Int i) -> Null {
//   print[+](%arg_0)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   return countup[?]((%arg_0 + 1))  # Null  #-> Void
// }
// fun chiika_main[?]() -> Null {
//   print[+](1)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](2)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](3)  #-> Null
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
//   print[+](1)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](2)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](3)  #-> Null
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
//   print[+](1)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](2)  #-> Null
//   sleep_sec[*](1)  #-> Null
//   print[+](3)  #-> Null
//   return null  # Null  #-> Void
// }
// 
// -- async_splitter output --
// extern print(Int n) -> Null;
// extern(async) sleep_sec(ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int n) -> RustFuture;
// fun countup[.](ChiikaEnv $env, (ChiikaEnv,Null)->RustFuture $cont, Int i) -> RustFuture {
//   chiika_env_push[.](%arg_0, FunToAny(%arg_1))  #-> Int
//   chiika_env_push[.](%arg_0, IntToAny(%arg_2))  #-> Int
//   print[+](%arg_2)  #-> Null
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
//   print[+](1)  #-> Null
//   return sleep_sec[.](%arg_0, chiika_main_1, 1)  # RustFuture  #-> Void
// }
// fun chiika_main_1[.](ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1  #-> Null
//   print[+](2)  #-> Null
//   return sleep_sec[.](%arg_0, chiika_main_2, 1)  # RustFuture  #-> Void
// }
// fun chiika_main_2[.](ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1  #-> Null
//   print[+](3)  #-> Null
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
//   print[+](%arg_2);  #-> Null
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
//   print[+](1);  #-> Null
//   return sleep_sec[.](%arg_0, chiika_main_1, 1)  # RustFuture;  #-> Void
// }
// fun chiika_main_1(ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1;  #-> Null
//   print[+](2);  #-> Null
//   return sleep_sec[.](%arg_0, chiika_main_2, 1)  # RustFuture;  #-> Void
// }
// fun chiika_main_2(ChiikaEnv $env, Null $async_result) -> RustFuture {
//   %arg_1;  #-> Null
//   print[+](3);  #-> Null
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
module {
  func.func private @print(i64) -> i64
  func.func private @sleep_sec(!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
  func.func private @chiika_env_push(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_pop(!llvm.ptr, i64) -> i64
  func.func private @chiika_env_ref(!llvm.ptr, i64) -> i64
  func.func private @chiika_start_tokio(i64) -> i64
  func.func @countup(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr, %arg2: i64) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %0 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %2 = call_indirect %f(%arg0, %1) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %3 = call_indirect %f_0(%arg0, %arg2) : (!llvm.ptr, i64) -> i64
    %f_1 = constant @print : (i64) -> i64
    %4 = call_indirect %f_1(%arg2) : (i64) -> i64
    %f_2 = constant @sleep_sec : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    %f_3 = constant @countup_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %5 = call_indirect %f_2(%arg0, %f_3, %c1_i64) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    return %5 : !llvm.ptr
  }
  func.func @countup_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @countup : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    %f_0 = constant @countup_2 : (!llvm.ptr, i64) -> !llvm.ptr
    %f_1 = constant @chiika_env_ref : (!llvm.ptr, i64) -> i64
    %c0_i64 = arith.constant 0 : i64
    %0 = call_indirect %f_1(%arg0, %c0_i64) : (!llvm.ptr, i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %1 = arith.addi %0, %c1_i64 : i64
    %2 = call_indirect %f(%arg0, %f_0, %1) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    return %2 : !llvm.ptr
  }
  func.func @countup_2(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @chiika_env_pop : (!llvm.ptr, i64) -> i64
    %c2_i64 = arith.constant 2 : i64
    %0 = call_indirect %f(%arg0, %c2_i64) : (!llvm.ptr, i64) -> i64
    %1 = llvm.inttoptr %0 : i64 to !llvm.ptr
    %2 = builtin.unrealized_conversion_cast %1 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %3 = call_indirect %2(%arg0, %arg1) : (!llvm.ptr, i64) -> !llvm.ptr
    return %3 : !llvm.ptr
  }
  func.func @chiika_main(%arg0: !llvm.ptr, %arg1: (!llvm.ptr, i64) -> !llvm.ptr) -> !llvm.ptr {
    %f = constant @chiika_env_push : (!llvm.ptr, i64) -> i64
    %0 = builtin.unrealized_conversion_cast %arg1 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %1 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %2 = call_indirect %f(%arg0, %1) : (!llvm.ptr, i64) -> i64
    %f_0 = constant @print : (i64) -> i64
    %c1_i64 = arith.constant 1 : i64
    %3 = call_indirect %f_0(%c1_i64) : (i64) -> i64
    %f_1 = constant @sleep_sec : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    %f_2 = constant @chiika_main_1 : (!llvm.ptr, i64) -> !llvm.ptr
    %c1_i64_3 = arith.constant 1 : i64
    %4 = call_indirect %f_1(%arg0, %f_2, %c1_i64_3) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    return %4 : !llvm.ptr
  }
  func.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %c2_i64 = arith.constant 2 : i64
    %0 = call_indirect %f(%c2_i64) : (i64) -> i64
    %f_0 = constant @sleep_sec : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    %f_1 = constant @chiika_main_2 : (!llvm.ptr, i64) -> !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %1 = call_indirect %f_0(%arg0, %f_1, %c1_i64) : (!llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr, i64) -> !llvm.ptr
    return %1 : !llvm.ptr
  }
  func.func @chiika_main_2(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %f = constant @print : (i64) -> i64
    %c3_i64 = arith.constant 3 : i64
    %0 = call_indirect %f(%c3_i64) : (i64) -> i64
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
