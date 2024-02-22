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
    %7 = llvm.mlir.constant(90 : i64) : i64
    %8 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    llvm.store %7, %8 : i64, !llvm.ptr
    %9 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    %10 = llvm.load %9 : !llvm.ptr -> i64
    %11 = llvm.mlir.constant(91 : i64) : i64
    %12 = llvm.icmp "eq" %10, %11 : i64
    llvm.cond_br %12, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %13 = llvm.mlir.addressof @putchar : !llvm.ptr
    %14 = llvm.mlir.constant(80 : i64) : i64
    %15 = llvm.mlir.constant(6 : i64) : i64
    %16 = llvm.mlir.constant(86 : i64) : i64
    %17 = llvm.call %13(%16) : !llvm.ptr, (i64) -> i64
    llvm.br ^bb3
  ^bb2:  // pred: ^bb0
    %18 = llvm.mlir.addressof @putchar : !llvm.ptr
    %19 = llvm.mlir.constant(80 : i64) : i64
    %20 = llvm.mlir.constant(6 : i64) : i64
    %21 = llvm.mlir.constant(74 : i64) : i64
    %22 = llvm.call %18(%21) : !llvm.ptr, (i64) -> i64
    llvm.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %23 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %23 : i64
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

