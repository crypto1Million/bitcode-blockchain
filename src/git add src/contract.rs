// src/contract.rs

use wasmer::{imports, Instance, Module, Store, Value, Function, FunctionEnv, FunctionEnvMut};
use wasmer::Value::I32;

pub fn execute_contract(wasm_bytes: &[u8], input: i32) -> Result<i32, String> {
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).map_err(|e| e.to_string())?;

    // Optional: define host functions (e.g., logging or state access)
    let env = FunctionEnv::new(&store, ());
    let import_object = imports! {
        "env" => {
            "log" => Function::new_typed_with_env(&store, &env, |_env: FunctionEnvMut<()>, val: i32| {
                println!("[WASM LOG] {}", val);
            }),
        }
    };

    let instance = Instance::new(&module, &import_object).map_err(|e| e.to_string())?;
    let run_func = instance.exports.get_function("run").map_err(|e| e.to_string())?;

    let result = run_func.call(&[Value::I32(input)])
        .map_err(|e| e.to_string())?;

    if let Some(I32(output)) = result.get(0) {
        Ok(*output)
    } else {
        Err("Unexpected return type from WASM".into())
    }
}
