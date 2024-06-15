; ModuleID = '<stdin>'
source_filename = "LLVMDialectModule"

declare i64 @print(i64) local_unnamed_addr

declare i64 @chiika_start_tokio(i64) local_unnamed_addr

define i64 @chiika_main() local_unnamed_addr {
  %1 = tail call i64 @print(i64 456)
  ret i64 0
}

define ptr @chiika_start_user(ptr %0, ptr nocapture readonly %1) local_unnamed_addr {
  %3 = tail call i64 @print(i64 456)
  %4 = tail call ptr %1(ptr %0, i64 0)
  ret ptr %4
}

define i64 @main() local_unnamed_addr {
  %1 = tail call i64 @chiika_start_tokio(i64 0)
  ret i64 0
}

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"Debug Info Version", i32 3}
