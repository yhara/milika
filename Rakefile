PREFIX = if RUBY_PLATFORM =~ /linux/
           "MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17"
         else
           ""
         end
SRC = Dir["src/**/*"]

file "a.mlir" => ["a.milika", *SRC] do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika > a.tmp 2>&1"
  s = File.read("a.tmp")
  File.write("a.mlir", s[/--CUTHERE--(.*)/m, 1])
end

file "a2.mlir" => ["a.mlir"] do
  sh "mlir-opt \
    --async-func-to-async-runtime \
    --async-to-async-runtime \
    --convert-async-to-llvm \
    --convert-arith-to-llvm \
    --convert-scf-to-cf \
    --convert-func-to-llvm \
    < a.mlir > a2.mlir"
end

file "a.ll" => ["a2.mlir"] do
  sh "mlir-translate --mlir-to-llvmir a2.mlir > a.ll"
end

task :default do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika"
end

task run: "a.ll" do
  sh "lli a.ll"
end

task a: :default

task :hand do
  sh "mlir-opt \
    --convert-arith-to-llvm \
    --convert-scf-to-cf \
    --convert-func-to-llvm \
    < a.mlir > a2.mlir"
    #--irdl-file=a.irdl \
    #--reconcile-unrealized-casts \
  sh "mlir-translate --mlir-to-llvmir a2.mlir > a.ll"
end
