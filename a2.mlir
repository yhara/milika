module attributes {llvm.data_layout = ""} {
  llvm.func @print(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @sleep_sec(!llvm.ptr, i64, !llvm.ptr) -> !llvm.ptr attributes {sym_visibility = "private"}
  llvm.func @chiika_env_push_frame(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_set(!llvm.ptr, i64, i64, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_pop_frame(!llvm.ptr, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_env_ref(!llvm.ptr, i64, i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_start_tokio(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @chiika_main(%arg0: !llvm.ptr, %arg1: !llvm.ptr) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @chiika_env_push_frame : !llvm.ptr
    %1 = llvm.mlir.constant(2 : i64) : i64
    %2 = llvm.call %0(%arg0, %1) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %3 = llvm.mlir.addressof @chiika_env_set : !llvm.ptr
    %4 = llvm.mlir.constant(0 : i64) : i64
    %5 = llvm.ptrtoint %arg1 : !llvm.ptr to i64
    %6 = llvm.mlir.constant(6 : i64) : i64
    %7 = llvm.call %3(%arg0, %4, %5, %6) : !llvm.ptr, (!llvm.ptr, i64, i64, i64) -> i64
    %8 = llvm.mlir.addressof @chiika_env_set : !llvm.ptr
    %9 = llvm.mlir.constant(1 : i64) : i64
    %10 = llvm.mlir.constant(3 : i64) : i64
    %11 = llvm.mlir.constant(1 : i64) : i64
    %12 = llvm.call %8(%arg0, %9, %10, %11) : !llvm.ptr, (!llvm.ptr, i64, i64, i64) -> i64
    %13 = llvm.mlir.addressof @print : !llvm.ptr
    %14 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %15 = llvm.mlir.constant(1 : i64) : i64
    %16 = llvm.mlir.constant(1 : i64) : i64
    %17 = llvm.call %14(%arg0, %15, %16) : !llvm.ptr, (!llvm.ptr, i64, i64) -> i64
    %18 = llvm.call %13(%17) : !llvm.ptr, (i64) -> i64
    %19 = llvm.mlir.addressof @sleep_sec : !llvm.ptr
    %20 = llvm.mlir.constant(1 : i64) : i64
    %21 = llvm.mlir.addressof @chiika_main_1 : !llvm.ptr
    %22 = llvm.call %19(%arg0, %20, %21) : !llvm.ptr, (!llvm.ptr, i64, !llvm.ptr) -> !llvm.ptr
    llvm.return %22 : !llvm.ptr
  }
  llvm.func @chiika_main_1(%arg0: !llvm.ptr, %arg1: i64) -> !llvm.ptr {
    %0 = llvm.mlir.addressof @print : !llvm.ptr
    %1 = llvm.mlir.addressof @chiika_env_ref : !llvm.ptr
    %2 = llvm.mlir.constant(1 : i64) : i64
    %3 = llvm.mlir.constant(1 : i64) : i64
    %4 = llvm.call %1(%arg0, %2, %3) : !llvm.ptr, (!llvm.ptr, i64, i64) -> i64
    %5 = llvm.call %0(%4) : !llvm.ptr, (i64) -> i64
    %6 = llvm.mlir.constant(1 : index) : i64
    %7 = llvm.alloca %6 x i64 : (i64) -> !llvm.ptr
    %8 = llvm.mlir.undef : !llvm.struct<(ptr, ptr, i64)>
    %9 = llvm.insertvalue %7, %8[0] : !llvm.struct<(ptr, ptr, i64)> 
    %10 = llvm.insertvalue %7, %9[1] : !llvm.struct<(ptr, ptr, i64)> 
    %11 = llvm.mlir.constant(0 : index) : i64
    %12 = llvm.insertvalue %11, %10[2] : !llvm.struct<(ptr, ptr, i64)> 
    %13 = llvm.mlir.constant(0 : i64) : i64
    llvm.store %13, %7 : i64, !llvm.ptr
    %14 = llvm.mlir.addressof @chiika_env_pop_frame : !llvm.ptr
    %15 = llvm.mlir.constant(2 : i64) : i64
    %16 = llvm.call %14(%arg0, %15) : !llvm.ptr, (!llvm.ptr, i64) -> i64
    %17 = llvm.inttoptr %16 : i64 to !llvm.ptr
    %18 = llvm.load %7 : !llvm.ptr -> i64
    %19 = llvm.call %17(%arg0, %18) : !llvm.ptr, (!llvm.ptr, i64) -> !llvm.ptr
    llvm.return %19 : !llvm.ptr
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

