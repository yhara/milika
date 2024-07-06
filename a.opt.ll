; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare ptr @sleep_sec(ptr, i64, ptr) local_unnamed_addr

declare i64 @chiika_env_push_frame(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_set(ptr, i64, i64, i64) local_unnamed_addr

declare i64 @chiika_env_pop_frame(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_ref(ptr, i64, i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define ptr @chiika_main(ptr %0, ptr %1) local_unnamed_addr {
  %3 = tail call i64 @chiika_env_push_frame(ptr %0, i64 2)
  %4 = ptrtoint ptr %1 to i64
  %5 = tail call i64 @chiika_env_set(ptr %0, i64 0, i64 %4, i64 6)
  %6 = tail call i64 @chiika_env_set(ptr %0, i64 1, i64 3, i64 1)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 1, i64 1)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @chiika_main_1)
  ret ptr %9
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 1, i64 1)
  %4 = tail call i64 @print(i64 %3)
  %5 = tail call i64 @chiika_env_pop_frame(ptr %0, i64 2)
  %6 = inttoptr i64 %5 to ptr
  %7 = tail call ptr %6(ptr %0, i64 0)
  ret ptr %7
}

define ptr @chiika_start_user(ptr %0, ptr %1) local_unnamed_addr {
  %3 = tail call i64 @chiika_env_push_frame(ptr %0, i64 2)
  %4 = ptrtoint ptr %1 to i64
  %5 = tail call i64 @chiika_env_set(ptr %0, i64 0, i64 %4, i64 6)
  %6 = tail call i64 @chiika_env_set(ptr %0, i64 1, i64 3, i64 1)
  %7 = tail call i64 @chiika_env_ref(ptr %0, i64 1, i64 1)
  %8 = tail call i64 @print(i64 %7)
  %9 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @chiika_main_1)
  ret ptr %9
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
