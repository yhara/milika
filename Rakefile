PREFIX = if RUBY_PLATFORM =~ /linux/
           "MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17"
         else
           ""
         end
SRC = Dir["src/**/*"]

#file "a.mlir" => ["a.milika", *SRC] do
#  sh "cargo fmt"
#  sh "#{PREFIX} cargo run -- a.milika > a.mlir"
#end
#
#file "a.ll" => ["a.mlir"] do
#  sh "mlir-translate --mlir-to-llvmir a.mlir > a.ll"
#end

task :default do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika"
end

task :run do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika > a.tmp 2>&1"
  s = File.read("a.tmp")
  File.write("a.mlir", s[/--CUTHERE--(.*)/m, 1])
  sh "mlir-translate --mlir-to-llvmir a.mlir > a.ll"
  sh "lli a.ll"
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
