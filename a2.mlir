module attributes {llvm.data_layout = ""} {
  llvm.func @putchar(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @main() -> i64 attributes {sym_visibility = "private"} {
    %0 = llvm.mlir.constant(1 : index) : i64
    %1 = llvm.alloca %0 x i64 : (i64) -> !llvm.ptr
    %2 = llvm.mlir.undef : !llvm.struct<(ptr, ptr, i64)>
    %3 = llvm.insertvalue %1, %2[0] : !llvm.struct<(ptr, ptr, i64)> 
    %4 = llvm.insertvalue %1, %3[1] : !llvm.struct<(ptr, ptr, i64)> 
    %5 = llvm.mlir.constant(0 : index) : i64
    %6 = llvm.insertvalue %5, %4[2] : !llvm.struct<(ptr, ptr, i64)> 
    %7 = llvm.mlir.constant(1 : i64) : i64
    %8 = llvm.mlir.constant(1 : i64) : i64
    %9 = llvm.mlir.constant(false) : i1
    llvm.cond_br %9, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %10 = llvm.mlir.addressof @putchar : !llvm.ptr
    %11 = llvm.mlir.constant(80 : i64) : i64
    %12 = llvm.mlir.constant(6 : i64) : i64
    %13 = llvm.mlir.constant(86 : i64) : i64
    %14 = llvm.call %10(%13) : !llvm.ptr, (i64) -> i64
    llvm.br ^bb3
  ^bb2:  // pred: ^bb0
    %15 = llvm.mlir.addressof @putchar : !llvm.ptr
    %16 = llvm.mlir.constant(80 : i64) : i64
    %17 = llvm.mlir.constant(6 : i64) : i64
    %18 = llvm.mlir.constant(74 : i64) : i64
    %19 = llvm.call %15(%18) : !llvm.ptr, (i64) -> i64
    llvm.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %20 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %20 : i64
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

