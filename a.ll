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

define ptr @foo(ptr %0, i64 %1, i64 %2, ptr %3) {
  %5 = ptrtoint ptr %3 to i64
  %6 = call i64 @chiika_env_push(ptr %0, i64 %5)
  %7 = call i64 @chiika_env_push(ptr %0, i64 %2)
  %8 = call i64 @chiika_env_push(ptr %0, i64 %1)
  %9 = icmp ult i64 %1, %2
  br i1 %9, label %10, label %12

10:                                               ; preds = %4
  %11 = call ptr @"foo't"(ptr %0)
  ret ptr %11

12:                                               ; preds = %4
  %13 = call ptr @"foo'f"(ptr %0)
  ret ptr %13
}

define ptr @"foo't"(ptr %0) {
  %2 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %3 = call i64 @chiika_env_ref(ptr %0, i64 1)
  %4 = add i64 %2, %3
  %5 = call i64 @print(i64 %4)
  %6 = call ptr @sleep_sec(ptr %0, i64 0, ptr @foo_2)
  ret ptr %6
}

define ptr @foo_2(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_pop(ptr %0, i64 3)
  %4 = inttoptr i64 %3 to ptr
  %5 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %6 = call i64 @chiika_env_ref(ptr %0, i64 1)
  %7 = add i64 %5, %6
  %8 = call ptr %4(ptr %0, i64 %7)
  ret ptr %8
}

define ptr @"foo'f"(ptr %0) {
  %2 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %3 = call i64 @chiika_env_ref(ptr %0, i64 1)
  %4 = add i64 %2, %3
  %5 = call i64 @print(i64 %4)
  %6 = call ptr @sleep_sec(ptr %0, i64 0, ptr @foo_4)
  ret ptr %6
}

define ptr @foo_4(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_pop(ptr %0, i64 3)
  %4 = inttoptr i64 %3 to ptr
  %5 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %6 = call i64 @chiika_env_ref(ptr %0, i64 1)
  %7 = add i64 %5, %6
  %8 = call ptr %4(ptr %0, i64 %7)
  ret ptr %8
}

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = ptrtoint ptr %1 to i64
  %4 = call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = call ptr @foo(ptr %0, i64 1, i64 2, ptr @chiika_main_1)
  ret ptr %5
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = call i64 @print(i64 %1)
  %4 = call i64 @chiika_env_pop(ptr %0, i64 1)
  %5 = inttoptr i64 %4 to ptr
  %6 = call ptr %5(ptr %0, i64 0)
  ret ptr %6
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
