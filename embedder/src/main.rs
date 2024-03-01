use std::{fs::File, io::Read};
use wasmer::{Store, Module, Instance, Value, imports};

static PLUGIN_PATH: &'static str = "./target/wasm32-unknown-unknown/release/wasm_plugin.wasm";

fn main() {
    let mut f = File::open(PLUGIN_PATH).unwrap();
    
    let mut wasm_plugin = Vec::default();
    f.read_to_end(&mut wasm_plugin).unwrap();

    let mut store = Store::default();
    let module = Module::new(&store, &wasm_plugin).unwrap();

    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let add = instance.exports.get_function("add").unwrap();
    let result = add.call(&mut store, &[Value::I32(1), Value::I32(2)]).unwrap();
    assert_eq!(result[0], Value::I32(3));
}
