module attributes {llvm.data_layout = ""} {
  llvm.func @print(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_push(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_pop(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_ref(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_start_tokio(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_main() -> i64 {
    %0 = llvm.mlir.constant(true) : i1
    llvm.cond_br %0, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %1 = llvm.mlir.addressof @print : !llvm.ptr
    %2 = llvm.mlir.constant(456 : i64) : i64
    %3 = llvm.call %1(%2) : !llvm.ptr, (i64) -> i64
    %4 = llvm.mlir.constant(0 : i64) : i64
    llvm.br ^bb3(%4 : i64)
  ^bb2:  // pred: ^bb0
    %5 = llvm.mlir.addressof @print : !llvm.ptr
    %6 = llvm.mlir.constant(789 : i64) : i64
    %7 = llvm.call %5(%6) : !llvm.ptr, (i64) -> i64
    %8 = llvm.mlir.constant(0 : i64) : i64
    llvm.br ^bb3(%8 : i64)
  ^bb3(%9: i64):  // 2 preds: ^bb1, ^bb2
    %10 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %10 : i64
  }
  llvm.func @chiika_start_user(%arg0: !llvm.ptr, %arg1: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_main : !llvm.ptr
    %1 = llvm.call %0() : !llvm.ptr, () -> i64
    %2 = llvm.call %arg1(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %2 : !llvm.ptr
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

