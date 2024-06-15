module attributes {llvm.data_layout = ""} {
  llvm.func @print(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @sleep_sec(!llvm.ptr, i64, !llvm.ptr) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @chiika_env_push(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_pop(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_ref(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_start_tokio(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @foo(%arg0: !llvm.ptr, %arg1: i64, %arg2: i64, %arg3: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_env_push : !llvm.ptr
    %1 = llvm.ptrtoint %arg3 : !llvm.ptr to i64
    %2 = llvm.call %0(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %3 = llvm.mlir.addressof @chiika_env_push : !llvm.ptr
    %4 = llvm.call %3(%arg0, %arg2) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %5 = llvm.mlir.addressof @chiika_env_push : !llvm.ptr
    %6 = llvm.call %5(%arg0, %arg1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %7 = llvm.icmp "ult" %arg1, %arg2 : i64
    llvm.cond_br %7, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %8 = llvm.mlir.addressof @"foo't" : !llvm.ptr
    %9 = llvm.call %8(%arg0) : !llvm.ptr, (!llvm.ptr) -> !llvm.ptr
    llvm.return %9 : !llvm.ptr
  ^bb2:  // pred: ^bb0
    %10 = llvm.mlir.addressof @"foo'f" : !llvm.ptr
    %11 = llvm.call %10(%arg0) : !llvm.ptr, (!llvm.ptr) -> !llvm.ptr
    llvm.return %11 : !llvm.ptr
  }
  llvm.func @"foo't"(%arg0: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @print : !llvm.ptr
    %1 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %2 = llvm.mlir.constant(0 : i64) : i64
    %3 = llvm.call %1(%arg0, %2) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %4 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %5 = llvm.mlir.constant(1 : i64) : i64
    %6 = llvm.call %4(%arg0, %5) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %7 = llvm.add %3, %6  : i64
    %8 = llvm.call %0(%7) : !llvm.ptr, (i64) -> i64
    %9 = llvm.mlir.addressof @sleep_sec : !llvm.ptr
    %10 = llvm.mlir.constant(0 : i64) : i64
    %11 = llvm.mlir.addressof @foo_2 : !llvm.ptr
    %12 = llvm.call %9(%arg0, %10, %11) : !llvm.ptr, (!llvm.ptr, i64, !llvm.ptr) -> !llvm.ptr
    llvm.return %12 : !llvm.ptr
  }
  llvm.func @foo_2(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_env_pop : !llvm.ptr
    %1 = llvm.mlir.constant(3 : i64) : i64
    %2 = llvm.call %0(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %3 = llvm.inttoptr %2 : i64 to !llvm.ptr
    %4 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %5 = llvm.mlir.constant(0 : i64) : i64
    %6 = llvm.call %4(%arg0, %5) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %7 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %8 = llvm.mlir.constant(1 : i64) : i64
    %9 = llvm.call %7(%arg0, %8) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %10 = llvm.add %6, %9  : i64
    %11 = llvm.call %3(%arg0, %10) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %11 : !llvm.ptr
  }
  llvm.func @"foo'f"(%arg0: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @print : !llvm.ptr
    %1 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %2 = llvm.mlir.constant(0 : i64) : i64
    %3 = llvm.call %1(%arg0, %2) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %4 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %5 = llvm.mlir.constant(1 : i64) : i64
    %6 = llvm.call %4(%arg0, %5) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %7 = llvm.add %3, %6  : i64
    %8 = llvm.call %0(%7) : !llvm.ptr, (i64) -> i64
    %9 = llvm.mlir.addressof @sleep_sec : !llvm.ptr
    %10 = llvm.mlir.constant(0 : i64) : i64
    %11 = llvm.mlir.addressof @foo_4 : !llvm.ptr
    %12 = llvm.call %9(%arg0, %10, %11) : !llvm.ptr, (!llvm.ptr, i64, !llvm.ptr) -> !llvm.ptr
    llvm.return %12 : !llvm.ptr
  }
  llvm.func @foo_4(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_env_pop : !llvm.ptr
    %1 = llvm.mlir.constant(3 : i64) : i64
    %2 = llvm.call %0(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %3 = llvm.inttoptr %2 : i64 to !llvm.ptr
    %4 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %5 = llvm.mlir.constant(0 : i64) : i64
    %6 = llvm.call %4(%arg0, %5) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %7 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %8 = llvm.mlir.constant(1 : i64) : i64
    %9 = llvm.call %7(%arg0, %8) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %10 = llvm.add %6, %9  : i64
    %11 = llvm.call %3(%arg0, %10) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %11 : !llvm.ptr
  }
  llvm.func @chiika_main(%arg0: !llvm.ptr, %arg1: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_env_push : !llvm.ptr
    %1 = llvm.ptrtoint %arg1 : !llvm.ptr to i64
    %2 = llvm.call %0(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %3 = llvm.mlir.addressof @foo : !llvm.ptr
    %4 = llvm.mlir.constant(1 : i64) : i64
    %5 = llvm.mlir.constant(2 : i64) : i64
    %6 = llvm.mlir.addressof @chiika_main_1 : !llvm.ptr
    %7 = llvm.call %3(%arg0, %4, %5, %6) : !llvm.ptr, (!llvm.ptr, i64, i64, !llvm.ptr) -> !llvm.ptr
    llvm.return %7 : !llvm.ptr
  }
  llvm.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @print : !llvm.ptr
    %1 = llvm.call %0(%arg1) : !llvm.ptr, (i64) -> i64
    %2 = llvm.mlir.addressof @chiika_env_pop : !llvm.ptr
    %3 = llvm.mlir.constant(1 : i64) : i64
    %4 = llvm.call %2(%arg0, %3) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %5 = llvm.inttoptr %4 : i64 to !llvm.ptr
    %6 = llvm.mlir.constant(0 : i64) : i64
    %7 = llvm.call %5(%arg0, %6) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %7 : !llvm.ptr
  }
  llvm.func @chiika_start_user(%arg0: !llvm.ptr, %arg1: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_main : !llvm.ptr
    %1 = llvm.call %0(%arg0, %arg1) : !llvm.ptr, (!llvm.ptr, !llvm.ptr) -> !llvm.ptr
    llvm.return %1 : !llvm.ptr
  }
  llvm.func @main() -> i64 {
    %0 = llvm.mlir.addressof @chiika_start_tokio : !llvm.ptr
    %1 = llvm.mlir.constant(0 : i64) : i64
    %2 = llvm.call %0(%1) : !llvm.ptr, (i64) -> i64
    %3 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %3 : i64
  }
  llvm.func @mlirAsyncRuntimeAddRef(!llvm.ptr, i64) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeDropRef(!llvm.ptr, i64) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeCreateToken() -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeCreateValue(i64) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeCreateGroup(i64) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeEmplaceToken(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeEmplaceValue(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeSetTokenError(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeSetValueError(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeIsTokenError(!llvm.ptr) -> i1 attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeIsValueError(!llvm.ptr) -> i1 attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeIsGroupError(!llvm.ptr) -> i1 attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitToken(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitValue(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitAllInGroup(!llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeExecute(!llvm.ptr, !llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeGetValueStorage(!llvm.ptr) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAddTokenToGroup(!llvm.ptr, !llvm.ptr) -> i64 attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitTokenAndExecute(!llvm.ptr, !llvm.ptr, !llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitValueAndExecute(!llvm.ptr, !llvm.ptr, !llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimeAwaitAllInGroupAndExecute(!llvm.ptr, !llvm.ptr, !llvm.ptr) attributes {sym_visibility = "private"}
  llvm.func @mlirAsyncRuntimGetNumWorkerThreads() -> i64 attributes {sym_visibility = "private"}
}

