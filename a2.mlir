module attributes {llvm.data_layout = ""} {
  llvm.func @free(!llvm.ptr)
  llvm.func @aligned_alloc(i64, i64) -> !llvm.ptr
  llvm.func @putchar(i64) -> i64 attributes {sym_visibility = "private"}
  llvm.func @main() -> i64 attributes {sym_visibility = "private"} {
    %0 = llvm.call @async_execute_fn() : () -> !llvm.struct<(ptr, ptr)>
    %1 = llvm.extractvalue %0[0] : !llvm.struct<(ptr, ptr)> 
    %2 = llvm.extractvalue %0[1] : !llvm.struct<(ptr, ptr)> 
    %3 = llvm.mlir.addressof @putchar : !llvm.ptr
    %4 = llvm.mlir.constant(68 : i64) : i64
    %5 = llvm.call %3(%4) : !llvm.ptr, (i64) -> i64
    %6 = llvm.mlir.addressof @putchar : !llvm.ptr
    %7 = llvm.mlir.constant(69 : i64) : i64
    %8 = llvm.call %6(%7) : !llvm.ptr, (i64) -> i64
    %9 = llvm.mlir.addressof @putchar : !llvm.ptr
    %10 = llvm.mlir.constant(70 : i64) : i64
    %11 = llvm.call %9(%10) : !llvm.ptr, (i64) -> i64
    %12 = llvm.mlir.constant(0 : i64) : i64
    llvm.return %12 : i64
  }
  llvm.func @async_execute_fn() -> !llvm.struct<(ptr, ptr)> attributes {passthrough = ["presplitcoroutine"], sym_visibility = "private"} {
    %0 = llvm.call @mlirAsyncRuntimeCreateToken() : () -> !llvm.ptr
    %1 = llvm.mlir.null : !llvm.ptr
    %2 = llvm.getelementptr %1[1] : (!llvm.ptr) -> !llvm.ptr, i64
    %3 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %4 = llvm.call @mlirAsyncRuntimeCreateValue(%3) : (i64) -> !llvm.ptr
    %5 = llvm.mlir.constant(0 : i32) : i32
    %6 = llvm.mlir.null : !llvm.ptr
    %7 = llvm.intr.coro.id %5, %6, %6, %6 : (i32, !llvm.ptr, !llvm.ptr, !llvm.ptr) -> !llvm.token
    %8 = llvm.intr.coro.size : i64
    %9 = llvm.intr.coro.align : i64
    %10 = llvm.add %8, %9  : i64
    %11 = llvm.mlir.constant(1 : i64) : i64
    %12 = llvm.sub %10, %11  : i64
    %13 = llvm.mlir.constant(0 : i64) : i64
    %14 = llvm.sub %13, %9  : i64
    %15 = llvm.and %12, %14  : i64
    %16 = llvm.call @aligned_alloc(%9, %15) : (i64, i64) -> !llvm.ptr
    %17 = llvm.intr.coro.begin %7, %16 : (!llvm.token, !llvm.ptr) -> !llvm.ptr
    %18 = llvm.intr.coro.save %17 : (!llvm.ptr) -> !llvm.token
    %19 = llvm.mlir.addressof @__resume : !llvm.ptr
    llvm.call @mlirAsyncRuntimeExecute(%17, %19) : (!llvm.ptr, !llvm.ptr) -> ()
    %20 = llvm.mlir.constant(false) : i1
    %21 = llvm.intr.coro.suspend %18, %20 : i8
    %22 = llvm.sext %21 : i8 to i32
    llvm.switch %22 : i32, ^bb3 [
      0: ^bb1,
      1: ^bb2
    ]
  ^bb1:  // pred: ^bb0
    %23 = llvm.mlir.addressof @putchar : !llvm.ptr
    %24 = llvm.mlir.constant(65 : i64) : i64
    %25 = llvm.call %23(%24) : !llvm.ptr, (i64) -> i64
    %26 = llvm.mlir.addressof @putchar : !llvm.ptr
    %27 = llvm.mlir.constant(66 : i64) : i64
    %28 = llvm.call %26(%27) : !llvm.ptr, (i64) -> i64
    %29 = llvm.mlir.addressof @putchar : !llvm.ptr
    %30 = llvm.mlir.constant(67 : i64) : i64
    %31 = llvm.call %29(%30) : !llvm.ptr, (i64) -> i64
    %32 = llvm.mlir.constant(0 : i64) : i64
    %33 = llvm.call @mlirAsyncRuntimeGetValueStorage(%4) : (!llvm.ptr) -> !llvm.ptr
    llvm.store %32, %33 : i64, !llvm.ptr
    llvm.call @mlirAsyncRuntimeEmplaceValue(%4) : (!llvm.ptr) -> ()
    llvm.call @mlirAsyncRuntimeEmplaceToken(%0) : (!llvm.ptr) -> ()
    llvm.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %34 = llvm.intr.coro.free %7, %17 : (!llvm.token, !llvm.ptr) -> !llvm.ptr
    llvm.call @free(%34) : (!llvm.ptr) -> ()
    llvm.br ^bb3
  ^bb3:  // 2 preds: ^bb0, ^bb2
    %35 = llvm.mlir.constant(false) : i1
    %36 = llvm.intr.coro.end %17, %35 : (!llvm.ptr, i1) -> i1
    %37 = llvm.mlir.undef : !llvm.struct<(ptr, ptr)>
    %38 = llvm.insertvalue %0, %37[0] : !llvm.struct<(ptr, ptr)> 
    %39 = llvm.insertvalue %4, %38[1] : !llvm.struct<(ptr, ptr)> 
    llvm.return %39 : !llvm.struct<(ptr, ptr)>
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
  llvm.func @__resume(%arg0: !llvm.ptr) attributes {sym_visibility = "private"} {
    llvm.intr.coro.resume %arg0 : !llvm.ptr
    llvm.return
  }
}

