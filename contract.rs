use wasmtime::*;
use std::collections::BTreeMap;

#[derive(Default, Debug, Clone)]
pub struct ContractState {
    pub storage: BTreeMap<String, String>,
}

impl ContractState {
    pub fn set(&mut self, key: &str, value: &str) {
        self.storage.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.get(key).cloned()
    }
}

pub struct WasmContractEngine {
    engine: Engine,
    linker: Linker<ContractState>,
}

impl WasmContractEngine {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        
        // Example: expose a host function to manipulate state
        linker.func_wrap("env", "set_value", |mut caller: Caller<'_, ContractState>, key_ptr: i32, key_len: i32, val_ptr: i32, val_len: i32| {
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
            let mut data = vec![0u8; (key_len + val_len) as usize];
            memory.read(&caller, key_ptr as usize, &mut data[..key_len as usize]).unwrap();
            memory.read(&caller, val_ptr as usize, &mut data[key_len as usize..]).unwrap();
            let key = String::from_utf8_lossy(&data[..key_len as usize]).to_string();
            let val = String::from_utf8_lossy(&data[key_len as usize..]).to_string();
            caller.data_mut().set(&key, &val);
            Ok(())
        }).unwrap();

        WasmContractEngine { engine, linker }
    }

    pub fn execute(&self, wasm_bytes: &[u8], state: &mut ContractState) -> Result<String, String> {
        let module = Module::new(&self.engine, wasm_bytes).map_err(|e| e.to_string())?;
        let mut store = Store::new(&self.engine, state.clone());
        let instance = self.linker.instantiate(&mut store, &module).map_err(|e| e.to_string())?;
        let func = instance.get_func(&mut store, "main").ok_or("Missing `main` entry point")?;
        let typed = func.typed::<(), (), _>(&store).map_err(|e| e.to_string())?;
        typed.call(&mut store, ()).map_err(|e| e.to_string())?;

        *state = store.data().clone();
        Ok("WASM executed successfully".into())
    }
}
