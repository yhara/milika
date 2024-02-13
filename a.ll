; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare ptr @aligned_alloc(i64, i64)

declare i64 @putchar(i64)

define i64 @main() {
  %1 = call { ptr, ptr } @async_execute_fn()
  %2 = extractvalue { ptr, ptr } %1, 0
  %3 = extractvalue { ptr, ptr } %1, 1
  %4 = call i64 @putchar(i64 68)
  %5 = call i64 @putchar(i64 69)
  %6 = call i64 @putchar(i64 70)
  ret i64 0
}

; Function Attrs: presplitcoroutine
define { ptr, ptr } @async_execute_fn() #0 {
  %1 = call ptr @mlirAsyncRuntimeCreateToken()
  %2 = call ptr @mlirAsyncRuntimeCreateValue(i64 ptrtoint (ptr getelementptr (i64, ptr null, i32 1) to i64))
  %3 = call token @llvm.coro.id(i32 0, ptr null, ptr null, ptr null)
  %4 = call i64 @llvm.coro.size.i64()
  %5 = call i64 @llvm.coro.align.i64()
  %6 = add i64 %4, %5
  %7 = sub i64 %6, 1
  %8 = sub i64 0, %5
  %9 = and i64 %7, %8
  %10 = call ptr @aligned_alloc(i64 %5, i64 %9)
  %11 = call ptr @llvm.coro.begin(token %3, ptr %10)
  %12 = call token @llvm.coro.save(ptr %11)
  call void @mlirAsyncRuntimeExecute(ptr %11, ptr @__resume)
  %13 = call i8 @llvm.coro.suspend(token %12, i1 false)
  %14 = sext i8 %13 to i32
  switch i32 %14, label %22 [
    i32 0, label %15
    i32 1, label %20
  ]

15:                                               ; preds = %0
  %16 = call i64 @putchar(i64 65)
  %17 = call i64 @putchar(i64 66)
  %18 = call i64 @putchar(i64 67)
  %19 = call ptr @mlirAsyncRuntimeGetValueStorage(ptr %2)
  store i64 0, ptr %19, align 4
  call void @mlirAsyncRuntimeEmplaceValue(ptr %2)
  call void @mlirAsyncRuntimeEmplaceToken(ptr %1)
  br label %20

20:                                               ; preds = %15, %0
  %21 = call ptr @llvm.coro.free(token %3, ptr %11)
  call void @free(ptr %21)
  br label %22

22:                                               ; preds = %20, %0
  %23 = call i1 @llvm.coro.end(ptr %11, i1 false)
  %24 = insertvalue { ptr, ptr } undef, ptr %1, 0
  %25 = insertvalue { ptr, ptr } %24, ptr %2, 1
  ret { ptr, ptr } %25
}

declare void @mlirAsyncRuntimeAddRef(ptr, i64)

declare void @mlirAsyncRuntimeDropRef(ptr, i64)

declare ptr @mlirAsyncRuntimeCreateToken()

declare ptr @mlirAsyncRuntimeCreateValue(i64)

declare ptr @mlirAsyncRuntimeCreateGroup(i64)

declare void @mlirAsyncRuntimeEmplaceToken(ptr)

declare void @mlirAsyncRuntimeEmplaceValue(ptr)

declare void @mlirAsyncRuntimeSetTokenError(ptr)

declare void @mlirAsyncRuntimeSetValueError(ptr)

declare i1 @mlirAsyncRuntimeIsTokenError(ptr)

declare i1 @mlirAsyncRuntimeIsValueError(ptr)

declare i1 @mlirAsyncRuntimeIsGroupError(ptr)

declare void @mlirAsyncRuntimeAwaitToken(ptr)

declare void @mlirAsyncRuntimeAwaitValue(ptr)

declare void @mlirAsyncRuntimeAwaitAllInGroup(ptr)

declare void @mlirAsyncRuntimeExecute(ptr, ptr)

declare ptr @mlirAsyncRuntimeGetValueStorage(ptr)

declare i64 @mlirAsyncRuntimeAddTokenToGroup(ptr, ptr)

declare void @mlirAsyncRuntimeAwaitTokenAndExecute(ptr, ptr, ptr)

declare void @mlirAsyncRuntimeAwaitValueAndExecute(ptr, ptr, ptr)

declare void @mlirAsyncRuntimeAwaitAllInGroupAndExecute(ptr, ptr, ptr)

declare i64 @mlirAsyncRuntimGetNumWorkerThreads()

define void @__resume(ptr %0) {
  call void @llvm.coro.resume(ptr %0)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: read)
declare token @llvm.coro.id(i32, ptr readnone, ptr nocapture readonly, ptr) #1

; Function Attrs: nounwind memory(none)
declare i64 @llvm.coro.size.i64() #2

; Function Attrs: nounwind memory(none)
declare i64 @llvm.coro.align.i64() #2

; Function Attrs: nounwind
declare ptr @llvm.coro.begin(token, ptr writeonly) #3

; Function Attrs: nomerge nounwind
declare token @llvm.coro.save(ptr) #4

; Function Attrs: nounwind
declare i8 @llvm.coro.suspend(token, i1) #3

; Function Attrs: nounwind memory(argmem: read)
declare ptr @llvm.coro.free(token, ptr nocapture readonly) #5

; Function Attrs: nounwind
declare i1 @llvm.coro.end(ptr, i1) #3

declare void @llvm.coro.resume(ptr)

attributes #0 = { presplitcoroutine }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(argmem: read) }
attributes #2 = { nounwind memory(none) }
attributes #3 = { nounwind }
attributes #4 = { nomerge nounwind }
attributes #5 = { nounwind memory(argmem: read) }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
