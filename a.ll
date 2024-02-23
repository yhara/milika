; ModuleID = 'LLVMDialectModule'
source_filename = "LLVMDialectModule"

declare ptr @malloc(i64)

declare void @free(ptr)

declare i64 @putchar(i64)

define i64 @main() {
  %1 = alloca i64, i64 1, align 8
  %2 = insertvalue { ptr, ptr, i64 } undef, ptr %1, 0
  %3 = insertvalue { ptr, ptr, i64 } %2, ptr %1, 1
  %4 = insertvalue { ptr, ptr, i64 } %3, i64 0, 2
  %5 = extractvalue { ptr, ptr, i64 } %4, 1
  store i64 0, ptr %5, align 4
  %6 = extractvalue { ptr, ptr, i64 } %4, 1
  %7 = load i64, ptr %6, align 4
  %8 = icmp ult i64 %7, 3
  br label %9

9:                                                ; preds = %13, %0
  %10 = extractvalue { ptr, ptr, i64 } %4, 1
  %11 = load i64, ptr %10, align 4
  %12 = icmp ult i64 %11, 3
  br i1 %12, label %13, label %22

13:                                               ; preds = %9
  %14 = extractvalue { ptr, ptr, i64 } %4, 1
  %15 = load i64, ptr %14, align 4
  %16 = add i64 %15, 80
  %17 = call i64 @putchar(i64 %16)
  %18 = extractvalue { ptr, ptr, i64 } %4, 1
  %19 = load i64, ptr %18, align 4
  %20 = add i64 %19, 1
  %21 = extractvalue { ptr, ptr, i64 } %4, 1
  store i64 %20, ptr %21, align 4
  br label %9

22:                                               ; preds = %9
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
