; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare ptr @sleep_sec(ptr, i64, ptr) local_unnamed_addr

declare i64 @chiika_env_push(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_pop(ptr, i64) local_unnamed_addr

declare i64 @chiika_env_ref(ptr, i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define ptr @foo(ptr %0, i64 %1, i64 %2, ptr %3) local_unnamed_addr {
common.ret:
  %4 = ptrtoint ptr %3 to i64
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 %4)
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 %2)
  %7 = tail call i64 @chiika_env_push(ptr %0, i64 %1)
  %8 = icmp ult i64 %1, %2
  %9 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %10 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %11 = add i64 %10, %9
  %12 = tail call i64 @print(i64 %11)
  %spec.select = select i1 %8, ptr @foo_2, ptr @foo_4
  %13 = tail call ptr @sleep_sec(ptr %0, i64 0, ptr nonnull %spec.select)
  ret ptr %13
}

define ptr @"foo't"(ptr %0) local_unnamed_addr {
  %2 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %4 = add i64 %3, %2
  %5 = tail call i64 @print(i64 %4)
  %6 = tail call ptr @sleep_sec(ptr %0, i64 0, ptr nonnull @foo_2)
  ret ptr %6
}

define ptr @foo_2(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_pop(ptr %0, i64 3)
  %4 = inttoptr i64 %3 to ptr
  %5 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %6 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %7 = add i64 %6, %5
  %8 = tail call ptr %4(ptr %0, i64 %7)
  ret ptr %8
}

define ptr @"foo'f"(ptr %0) local_unnamed_addr {
  %2 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %3 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %4 = add i64 %3, %2
  %5 = tail call i64 @print(i64 %4)
  %6 = tail call ptr @sleep_sec(ptr %0, i64 0, ptr nonnull @foo_4)
  ret ptr %6
}

define ptr @foo_4(ptr %0, i64 %1) {
  %3 = tail call i64 @chiika_env_pop(ptr %0, i64 3)
  %4 = inttoptr i64 %3 to ptr
  %5 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %6 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %7 = add i64 %6, %5
  %8 = tail call ptr %4(ptr %0, i64 %7)
  ret ptr %8
}

define ptr @chiika_main(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 ptrtoint (ptr @chiika_main_1 to i64))
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 2)
  %7 = tail call i64 @chiika_env_push(ptr %0, i64 1)
  %8 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %9 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %10 = add i64 %9, %8
  %11 = tail call i64 @print(i64 %10)
  %12 = tail call ptr @sleep_sec(ptr %0, i64 0, ptr nonnull @foo_2)
  ret ptr %12
}

define ptr @chiika_main_1(ptr %0, i64 %1) {
  %3 = tail call i64 @print(i64 %1)
  %4 = tail call i64 @chiika_env_pop(ptr %0, i64 1)
  %5 = inttoptr i64 %4 to ptr
  %6 = tail call ptr %5(ptr %0, i64 0)
  ret ptr %6
}

define ptr @chiika_start_user(ptr %0, ptr %1) local_unnamed_addr {
  %3 = ptrtoint ptr %1 to i64
  %4 = tail call i64 @chiika_env_push(ptr %0, i64 %3)
  %5 = tail call i64 @chiika_env_push(ptr %0, i64 ptrtoint (ptr @chiika_main_1 to i64))
  %6 = tail call i64 @chiika_env_push(ptr %0, i64 2)
  %7 = tail call i64 @chiika_env_push(ptr %0, i64 1)
  %8 = tail call i64 @chiika_env_ref(ptr %0, i64 0)
  %9 = tail call i64 @chiika_env_ref(ptr %0, i64 1)
  %10 = add i64 %9, %8
  %11 = tail call i64 @print(i64 %10)
  %12 = tail call ptr @sleep_sec(ptr %0, i64 0, ptr nonnull @foo_2)
  ret ptr %12
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
