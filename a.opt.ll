; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare ptr @sleep_sec(ptr, i64, ptr) local_unnamed_addr

declare i64 @chiika_env_push(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_pop(ptr, i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define ptr @chiika_main(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @chiika_main_1)
  ret ptr %5
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = tail call i64 @print(i64 123)
  %4 = tail call i64 @chiika_env_pop(ptr %0, i64 1)
  %5 = inttoptr i64 %4 to ptr
  %6 = tail call ptr %5(ptr %0, i64 0)
  ret ptr %6
}

define ptr @chiika_start_user(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call ptr @sleep_sec(ptr %0, i64 1, ptr nonnull @chiika_main_1)
  ret ptr %5
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
