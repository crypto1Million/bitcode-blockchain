(module
  ;; Memory & Imports
  (memory 1)
  (export "memory" (memory 0))
  (import "env" "log" (func $log (param i32)))

  ;; Data section
  (data (i32.const 0) "Bitcode-20 Running\00")

  ;; Function: run(i32) -> i32
  (func $run (export "run") (param $input i32) (result i32)
    ;; Call imported log function with pointer to string
    (call $log (i32.const 0))

    ;; Simulate balance check logic (mock)
    (i32.add (local.get $input) (i32.const 1000))
  )
)
