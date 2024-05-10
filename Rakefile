NAME = ENV["NAME"] || "a"
CARGO_TARGET = ENV["SHIIKA_CARGO_TARGET"] || "./chiika_runtime/target"
RUNTIME = Dir["chiika_runtime/**/*"]
RUNTIME_A = File.expand_path "#{CARGO_TARGET}/debug/libchiika_runtime.a"
PREFIX, SUFFIX =
  if File.exist?("/usr/lib/llvm-17")
    ["MLIR_SYS_170_PREFIX=/usr/lib/llvm-17 TABLEGEN_170_PREFIX=/usr/lib/llvm-17", "-17"]
  else
    ["", ""]
  end
SRC = Dir["src/**/*"]

task :clean do
  [".", "examples"].each do |dir|
    cd dir do
      sh "rm -f *.mlir *.tmp *.ll *.out *.actual_out"
    end
  end
end

def lowering(name)
  sh "mlir-opt#{SUFFIX} \
    --async-func-to-async-runtime \
    --async-to-async-runtime \
    --convert-async-to-llvm \
    --convert-arith-to-llvm \
    --convert-scf-to-cf \
    --convert-func-to-llvm \
    --finalize-memref-to-llvm \
    --reconcile-unrealized-casts \
    < #{name}.mlir > #{name}2.mlir"
end

file RUNTIME_A => [*RUNTIME] do
  Dir.chdir "chiika_runtime" do
    sh "cargo fmt"
    sh "cargo build"
  end
end

file "#{NAME}.mlir" => ["#{NAME}.milika", *SRC] do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- #{NAME}.milika > #{NAME}.tmp 2>&1" do |ok, status|
    unless ok
      sh "cat #{NAME}.tmp"
      raise "cargo run failed"
    end
  end
  s = File.read("#{NAME}.tmp")
  File.write("#{NAME}.mlir", s[/--CUTHERE--(.*)/m, 1])
end

file "#{NAME}2.mlir" => ["#{NAME}.mlir"] do
  lowering(NAME)
end

file "#{NAME}.ll" => ["#{NAME}2.mlir"] do
  sh "mlir-translate#{SUFFIX} --mlir-to-llvmir #{NAME}2.mlir > #{NAME}.ll"
end

file "#{NAME}.out" => [RUNTIME_A, "#{NAME}.ll"] do
  sh "clang#{SUFFIX}",
    "-lm",
    "-ldl",
    "-lpthread",
    "-o", "#{NAME}.out",
    "#{NAME}.ll",
    RUNTIME_A
end

task :default do
  sh "cargo fmt"
  sh "#{PREFIX} cargo run -- a.milika"
end

task run: "#{NAME}.out" do
  sh "./#{NAME}.out"
end

task a: :default

task :hand do
  sh "mlir-opt#{SUFFIX} \
    --convert-index-to-llvm \
    --convert-arith-to-llvm \
    --convert-scf-to-cf \
    --convert-func-to-llvm \
    --finalize-memref-to-llvm \
    < a.mlir > a2.mlir" 
    #--irdl-file=a.irdl \
    #--reconcile-unrealized-casts \
  sh "mlir-translate --mlir-to-llvmir a2.mlir > a.ll"
end

task :cpp do
  sh "clang++ -std=c++20 -c -emit-llvm a.cpp"
end

task :tmp do
  #sh %{mlir-opt#{SUFFIX} mlir-opt-async.mlir -pass-pipeline="builtin.module(async-to-async-runtime,func.func(async-runtime-ref-counting,async-runtime-ref-counting-opt),convert-async-to-llvm,func.func(convert-linalg-to-loops,convert-scf-to-cf),finalize-memref-to-llvm,func.func(convert-arith-to-llvm),convert-func-to-llvm,reconcile-unrealized-casts)" > b.mlir }
  #sh "mlir-translate#{SUFFIX} --mlir-to-llvmir b.mlir > b.ll"
  #lowering("b")
  sh "#{PREFIX} cargo run --example parser2"
end

task :integration_test do
  Dir["examples/*.milika"].each do |path|
    name = path.sub(".milika", "")
    sh "NAME=#{name} rake run > #{name}.actual_out"
    sh "diff #{name}.actual_out #{name}.expected_out"
  end
end
