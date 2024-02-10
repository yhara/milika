PREFIX = if RUBY_PLATFORM =~ /linux/
           "MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17"
         else
           ""
         end
task :default do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika"
end

task a: :default

task :hand do
  sh "mlir-opt \
    --irdl-file=a.irdl \
    --convert-arith-to-llvm \
    --convert-scf-to-cf \
    --convert-func-to-llvm \
    --reconcile-unrealized-casts \
    < a.mlir > a2.mlir"
  sh "mlir-translate --mlir-to-llvmir a2.mlir > a.ll"
end

task :llvm do
  sh "mlir-translate --mlir-to-llvmir c.mlir > c.ll"
end
