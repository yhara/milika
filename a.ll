; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @print(i64)

declare ptr @sleep_sec(ptr, i64, ptr)

declare i64 @chiika_env_push_frame(ptr, i64)

declare i64 @chiika_env_set(ptr, i64, i64, i64)

declare i64 @chiika_env_pop_frame(ptr, i64)

declare i64 @chiika_env_ref(ptr, i64, i64)

declare i64 @chiika_start_tokio(i64)

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = call i64 @chiika_env_push_frame(ptr %0, i64 2)
  %4 = ptrtoint ptr %1 to i64
  %5 = call i64 @chiika_env_set(ptr %0, i64 0, i64 %4, i64 6)
  %6 = call i64 @chiika_env_set(ptr %0, i64 1, i64 3, i64 1)
  %7 = call i64 @chiika_env_ref(ptr %0, i64 1, i64 1)
  %8 = call i64 @print(i64 %7)
  %9 = call ptr @sleep_sec(ptr %0, i64 1, ptr @chiika_main_1)
  ret ptr %9
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 1, i64 1)
  %4 = call i64 @print(i64 %3)
  %5 = alloca i64, i64 1, align 8
  %6 = insertvalue { ptr, ptr, i64 } undef, ptr %5, 0
  %7 = insertvalue { ptr, ptr, i64 } %6, ptr %5, 1
  %8 = insertvalue { ptr, ptr, i64 } %7, i64 0, 2
  store i64 0, ptr %5, align 4
  %9 = call i64 @chiika_env_pop_frame(ptr %0, i64 2)
  %10 = inttoptr i64 %9 to ptr
  %11 = load i64, ptr %5, align 4
  %12 = call ptr %10(ptr %0, i64 %11)
  ret ptr %12
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
