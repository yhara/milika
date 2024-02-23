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
    %7 = llvm.mlir.constant(0 : i64) : i64
    %8 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    llvm.store %7, %8 : i64, !llvm.ptr
    %9 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    %10 = llvm.load %9 : !llvm.ptr -> i64
    %11 = llvm.mlir.constant(3 : i64) : i64
    %12 = llvm.icmp "ult" %10, %11 : i64
    llvm.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb2
    %13 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    %14 = llvm.load %13 : !llvm.ptr -> i64
    %15 = llvm.mlir.constant(3 : i64) : i64
    %16 = llvm.icmp "ult" %14, %15 : i64
    llvm.cond_br %16, ^bb2, ^bb3
  ^bb2:  // pred: ^bb1
    %17 = llvm.mlir.addressof @putchar : !llvm.ptr
    %18 = llvm.mlir.constant(80 : i64) : i64
    %19 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    %20 = llvm.load %19 : !llvm.ptr -> i64
    %21 = llvm.add %20, %18  : i64
    %22 = llvm.call %17(%21) : !llvm.ptr, (i64) -> i64
    %23 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    %24 = llvm.load %23 : !llvm.ptr -> i64
    %25 = llvm.mlir.constant(1 : i64) : i64
    %26 = llvm.add %24, %25  : i64
    %27 = llvm.extractvalue %6[1] : !llvm.struct<(ptr, ptr, i64)> 
    llvm.store %26, %27 : i64, !llvm.ptr
    llvm.br ^bb1
  ^bb3:  // pred: ^bb1
    %28 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %28 : i64
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

