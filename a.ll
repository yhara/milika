; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @print(i64)

declare ptr @sleep_sec(ptr, ptr, i64)

declare i64 @chiika_env_push(ptr, i64)

declare i64 @chiika_env_pop(ptr, i64)

declare i64 @chiika_env_ref(ptr, i64)

declare i64 @chiika_start_tokio(i64)

define ptr @countup(ptr %0, ptr %1, i64 %2) {
  %4 = ptrtoint ptr %1 to i64
  %5 = call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = call i64 @chiika_env_push(ptr %0, i64 %2)
  %7 = call i64 @print(i64 %2)
  %8 = call ptr @sleep_sec(ptr %0, ptr @countup_1, i64 1)
  ret ptr %8
}

define ptr @countup_1(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = add i64 %3, 1
  %5 = call ptr @countup(ptr %0, ptr @countup_2, i64 %4)
  ret ptr %5
}

define ptr @countup_2(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_pop(ptr %0, i64 2)
  %4 = inttoptr i64 %3 to ptr
  %5 = call ptr %4(ptr %0, i64 %1)
  ret ptr %5
}

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = ptrtoint ptr %1 to i64
  %4 = call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = call i64 @print(i64 1)
  %6 = call ptr @sleep_sec(ptr %0, ptr @chiika_main_1, i64 1)
  ret ptr %6
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = call i64 @print(i64 2)
  %4 = call ptr @sleep_sec(ptr %0, ptr @chiika_main_2, i64 1)
  ret ptr %4
}

define ptr @chiika_main_2(ptr %0, i64 %1) {
  %3 = call i64 @print(i64 3)
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
