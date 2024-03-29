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
  %1 = call i64 @print(i64 123)
  br i1 true, label %2, label %4

2:                                                ; preds = %0
  %3 = call i64 @"chiika_main't"()
  br label %6

4:                                                ; preds = %0
  %5 = call i64 @"chiika_main'f"()
  br label %6

6:                                                ; preds = %2, %4
  %7 = phi i64 [ %5, %4 ], [ %3, %2 ]
  br label %8

8:                                                ; preds = %6
  ret i64 %7
}

define ptr @"chiika_main't"(ptr %0, ptr %1) {
  %3 = call i64 @chiika_env_push(ptr %0, ptr %1)
  %4 = call ptr @sleep_sec(ptr %0, ptr @"chiika_main't_1", i64 1)
  ret ptr %4
}

define ptr @"chiika_main't_1"(ptr %0, i64 %1) {
  %3 = call i64 @print(i64 456)
  %4 = call ptr @chiika_env_pop(ptr %0, i64 1)
  %5 = call i64 @"chiika_main'e"()
  %6 = call ptr %4(ptr %0, i64 %5)
  ret ptr %6
}

define i64 @"chiika_main'f"() {
  %1 = call i64 @print(i64 789)
  %2 = call i64 @"chiika_main'e"()
  ret i64 %2
}

define i64 @"chiika_main'e"() {
  %1 = call i64 @print(i64 0)
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
