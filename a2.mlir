module attributes {llvm.data_layout = ""} {
  llvm.func @putchar(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @sleep_sec(!llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @chiika_env_push(!llvm.ptr, !llvm.ptr) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_pop(!llvm.ptr, i64) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @chiika_env_ref(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_start_tokio(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_main(%arg0: !llvm.ptr, %arg1: !llvm.ptr) -> !llvm.ptr {
    %0 = builtin.unrealized_conversion_cast %arg1 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %1 = llvm.mlir.addressof @chiika_env_push : !llvm.ptr
    %2 = builtin.unrealized_conversion_cast %0 : (!llvm.ptr, i64) -> !llvm.ptr to !llvm.ptr
    %3 = llvm.call %1(%arg0, %2) : !llvm.ptr, (!llvm.ptr, !llvm.ptr) -> i64
    %4 = llvm.mlir.addressof @putchar : !llvm.ptr
    %5 = llvm.mlir.constant(70 : i64) : i64
    %6 = llvm.call %4(%5) : !llvm.ptr, (i64) -> i64
    %7 = llvm.mlir.addressof @sleep_sec : !llvm.ptr
    %8 = llvm.mlir.addressof @chiika_main_1 : !llvm.ptr
    %9 = llvm.mlir.constant(1 : i64) : i64
    %10 = llvm.call %7(%arg0, %8, %9) : !llvm.ptr, (!llvm.ptr, !llvm.ptr, i64) -> !llvm.ptr
    llvm.return %10 : !llvm.ptr
  }
  llvm.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @putchar : !llvm.ptr
    %1 = llvm.mlir.constant(72 : i64) : i64
    %2 = llvm.call %0(%1) : !llvm.ptr, (i64) -> i64
    %3 = llvm.mlir.addressof @chiika_env_pop : !llvm.ptr
    %4 = llvm.mlir.constant(1 : i64) : i64
    %5 = llvm.call %3(%arg0, %4) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    %6 = builtin.unrealized_conversion_cast %5 : !llvm.ptr to (!llvm.ptr, i64) -> !llvm.ptr
    %7 = llvm.mlir.constant(0 : i64) : i64
    %8 = llvm.call %5(%arg0, %7) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %8 : !llvm.ptr
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

