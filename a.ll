; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @print(i64)

declare ptr @sleep_sec(ptr, ptr, i64)

declare i64 @chiika_env_push(ptr, ptr)

declare ptr @chiika_env_pop(ptr, i64)

declare i64 @chiika_env_ref(ptr, i64)

declare i64 @chiika_start_tokio(i64)

define i64 @chiika_main() {
  br i1 true, label %1, label %3

1:                                                ; preds = %0
  %2 = call i64 @print(i64 456)
  br label %5

3:                                                ; preds = %0
  %4 = call i64 @print(i64 789)
  br label %5

5:                                                ; preds = %1, %3
  ret i64 0
}

define ptr @chiika_start_user(ptr %0, ptr %1) {
  %3 = call i64 @chiika_main()
  %4 = call ptr %1(ptr %0, i64 %3)
  ret ptr %4
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
