; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @print(i64)

declare ptr @sleep_sec(ptr, i64, ptr)

declare i64 @chiika_env_push(ptr, i64)

declare i64 @chiika_env_pop(ptr, i64)

declare i64 @chiika_env_ref(ptr, i64)

declare i64 @chiika_start_tokio(i64)

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = ptrtoint ptr %1 to i64
  %4 = call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = call ptr @sleep_sec(ptr %0, i64 1, ptr @chiika_main_1)
  ret ptr %5
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  br i1 true, label %3, label %5

3:                                                ; preds = %2
  %4 = call i64 @print(i64 123)
  br label %6

5:                                                ; preds = %2
  br label %6

6:                                                ; preds = %3, %5
  %7 = phi i64 [ 0, %5 ], [ 0, %3 ]
  %8 = call i64 @chiika_env_pop(ptr %0, i64 1)
  %9 = inttoptr i64 %8 to ptr
  %10 = call ptr %9(ptr %0, i64 0)
  ret ptr %10
}

define ptr @chiika_start_user(ptr %0, ptr %1) {
  %3 = call ptr @chiika_main(ptr %0, ptr %1)
  ret ptr %3
}

define i64 @main() {
  %1 = call i64 @chiika_start_tokio(i64 0)
  ret i64 0
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

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
