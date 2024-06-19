; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @print(i64)

declare ptr @sleep_sec(ptr, i64, ptr)

declare i64 @chiika_env_push(ptr, i64, i64)

declare i64 @chiika_env_pop(ptr, i64)

declare i64 @chiika_env_ref(ptr, i64, i64)

declare i64 @chiika_start_tokio(i64)

define ptr @countdown(ptr %0, i64 %1, ptr %2) {
  %4 = ptrtoint ptr %2 to i64
  %5 = call i64 @chiika_env_push(ptr %0, i64 %4, i64 6)
  %6 = call i64 @chiika_env_push(ptr %0, i64 %1, i64 1)
  %7 = call i64 @print(i64 %1)
  %8 = call ptr @sleep_sec(ptr %0, i64 0, ptr @countdown_1)
  ret ptr %8
}

define ptr @countdown_1(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 0, i64 1)
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %5, label %7

5:                                                ; preds = %2
  %6 = call ptr @"countdown_1't"(ptr %0)
  ret ptr %6

7:                                                ; preds = %2
  %8 = call ptr @"countdown_1'f"(ptr %0)
  ret ptr %8
}

define ptr @"countdown_1't"(ptr %0) {
  %2 = alloca i64, i64 1, align 8
  %3 = insertvalue { ptr, ptr, i64 } undef, ptr %2, 0
  %4 = insertvalue { ptr, ptr, i64 } %3, ptr %2, 1
  %5 = insertvalue { ptr, ptr, i64 } %4, i64 0, 2
  store i64 0, ptr %2, align 4
  %6 = call i64 @chiika_env_pop(ptr %0, i64 2)
  %7 = inttoptr i64 %6 to ptr
  %8 = load i64, ptr %2, align 4
  %9 = call ptr %7(ptr %0, i64 %8)
  ret ptr %9
}

define ptr @"countdown_1'f"(ptr %0) {
  %2 = call ptr @"countdown_1'e"(ptr %0, i64 0)
  ret ptr %2
}

define ptr @"countdown_1'e"(ptr %0, i64 %1) {
  %3 = call i64 @chiika_env_ref(ptr %0, i64 0, i64 1)
  %4 = sub i64 %3, 1
  %5 = call i64 @chiika_env_pop(ptr %0, i64 2)
  %6 = inttoptr i64 %5 to ptr
  %7 = call ptr @countdown(ptr %0, i64 %4, ptr %6)
  ret ptr %7
}

define ptr @chiika_main(ptr %0, ptr %1) {
  %3 = ptrtoint ptr %1 to i64
  %4 = call i64 @chiika_env_push(ptr %0, i64 %3, i64 6)
  %5 = call ptr @countdown(ptr %0, i64 3, ptr @chiika_main_1)
  ret ptr %5
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = alloca i64, i64 1, align 8
  %4 = insertvalue { ptr, ptr, i64 } undef, ptr %3, 0
  %5 = insertvalue { ptr, ptr, i64 } %4, ptr %3, 1
  %6 = insertvalue { ptr, ptr, i64 } %5, i64 0, 2
  store i64 0, ptr %3, align 4
  %7 = call i64 @chiika_env_pop(ptr %0, i64 1)
  %8 = inttoptr i64 %7 to ptr
  %9 = load i64, ptr %3, align 4
  %10 = call ptr %8(ptr %0, i64 %9)
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
