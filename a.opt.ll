; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare ptr @sleep_sec(ptr, ptr, i64) local_unnamed_addr

declare i64 @chiika_env_push(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_pop(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_ref(ptr, i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define ptr @countup(ptr %0, ptr %1, i64 %2) local_unnamed_addr {
  %4 = ptrtoint ptr %1 to i64
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %2)
  %7 = tail call i64 @print(i64 %2)
  %8 = tail call ptr @sleep_sec(ptr %0, ptr nonnull @countup_1, i64 1)
  ret ptr %8
}

define ptr @countup_1(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %4 = add i64 %3, 1
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 ptrtoint (ptr @countup_2 to i64))
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %7 = tail call i64 @print(i64 %4)
  %8 = tail call ptr @sleep_sec(ptr %0, ptr nonnull @countup_1, i64 1)
  ret ptr %8
}

define ptr @countup_2(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_pop(ptr %0, i64 2)
  %4 = inttoptr i64 %3 to ptr
  %5 = tail call ptr %4(ptr %0, i64 %1)
  ret ptr %5
}

define ptr @chiika_main(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @print(i64 1)
  %6 = tail call ptr @sleep_sec(ptr %0, ptr nonnull @chiika_main_1, i64 1)
  ret ptr %6
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = tail call i64 @print(i64 2)
  %4 = tail call ptr @sleep_sec(ptr %0, ptr nonnull @chiika_main_2, i64 1)
  ret ptr %4
}

define ptr @chiika_main_2(ptr %0, i64 %1) {
  %3 = tail call i64 @print(i64 3)
  %4 = tail call i64 @chiika_env_pop(ptr %0, i64 1)
  %5 = inttoptr i64 %4 to ptr
  %6 = tail call ptr %5(ptr %0, i64 0)
  ret ptr %6
}

define ptr @chiika_start_user(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @print(i64 1)
  %6 = tail call ptr @sleep_sec(ptr %0, ptr nonnull @chiika_main_1, i64 1)
  ret ptr %6
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
