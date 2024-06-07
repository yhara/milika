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

define ptr @countdown(ptr %0, i64 %1, ptr %2) {
  %4 = ptrtoint ptr %2 to i64
  %5 = call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = call i64 @chiika_env_push(ptr %0, i64 %1)
  %7 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %8 = call i64 @print(i64 %7)
  %9 = call ptr @sleep_sec(ptr %0, i64 1, ptr @countdown_1)
  ret ptr %9
}

define ptr @countdown_1(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %5, label %7

5:                                                ; preds = %2
  %6 = call ptr @"countdown't"(ptr %0)
  br label %9

7:                                                ; preds = %2
  %8 = call ptr @"countdown'f"(ptr %0)
  br label %9

9:                                                ; preds = %5, %7
  %10 = phi ptr [ %8, %7 ], [ %6, %5 ]
  ret ptr %10
}

define ptr @"countdown't"(ptr %0) {
  %2 = call i64 @chiika_env_pop(ptr %0, i64 2)
  %3 = inttoptr i64 %2 to ptr
  %4 = call ptr %3(ptr %0, i64 0)
  ret ptr %4
}

define ptr @"countdown'f"(ptr %0) {
  %2 = call ptr @"countdown'e"(ptr %0, i64 0)
  ret ptr %2
}

define ptr @"countdown'e"(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = sub i64 %3, 1
  %5 = call i64 @chiika_env_pop(ptr %0, i64 2)
  %6 = inttoptr i64 %5 to ptr
  %7 = call ptr @countdown(ptr %0, i64 %4, ptr %6)
  ret ptr %7
}

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = ptrtoint ptr %1 to i64
  %4 = call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = call ptr @countdown(ptr %0, i64 3, ptr @chiika_main_1)
  ret ptr %5
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_pop(ptr %0, i64 1)
  %4 = inttoptr i64 %3 to ptr
  %5 = call ptr %4(ptr %0, i64 0)
  ret ptr %5
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
