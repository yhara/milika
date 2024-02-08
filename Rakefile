task :default do
  #sh "MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17 cargo run"
  sh "cargo fmt"
  sh "cargo run -- a.milika"
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
