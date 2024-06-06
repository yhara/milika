; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare ptr @sleep_sec(ptr, i64, ptr) local_unnamed_addr

declare i64 @chiika_env_push(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_pop(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_ref(ptr, i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define ptr @countdown(ptr %0, i64 %1, ptr %2) local_unnamed_addr {
  %4 = ptrtoint ptr %2 to i64
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %1)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  ret ptr %9
}

define ptr @countdown_1(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %5, label %9

5:                                                ; preds = %2
  %6 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %7 = inttoptr i64 %6 to ptr
  %8 = tail call ptr %7(ptr %0, i64 0)
  br label %18

9:                                                ; preds = %2
  %10 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %11 = add i64 %10, -1
  %12 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %13 = tail call i64 @chiika_env_push(ptr %0, i64 %12)
  %14 = tail call i64 @chiika_env_push(ptr %0, i64 %11)
  %15 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %16 = tail call i64 @print(i64 %15)
  %17 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  br label %18

18:                                               ; preds = %9, %5
  %19 = phi ptr [ %17, %9 ], [ %8, %5 ]
  ret ptr %19
}

define ptr @"countdown't"(ptr %0) local_unnamed_addr {
  %2 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %3 = inttoptr i64 %2 to ptr
  %4 = tail call ptr %3(ptr %0, i64 0)
  ret ptr %4
}

define ptr @"countdown'f"(ptr %0) local_unnamed_addr {
  %2 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %3 = add i64 %2, -1
  %4 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  ret ptr %9
}

define ptr @"countdown'e"(ptr %0, i64 %1) local_unnamed_addr {
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = add i64 %3, -1
  %5 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %5)
  %7 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %8 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %9 = tail call i64 @print(i64 %8)
  %10 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  ret ptr %10
}

define ptr @chiika_main(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 ptrtoint (ptr @chiika_main_1 to i64))
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 3)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  ret ptr %9
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_pop(ptr %0, i64 1)
  %4 = inttoptr i64 %3 to ptr
  %5 = tail call ptr %4(ptr %0, i64 0)
  ret ptr %5
}

define ptr @chiika_start_user(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 ptrtoint (ptr @chiika_main_1 to i64))
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 3)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @countdown_1)
  ret ptr %9
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
