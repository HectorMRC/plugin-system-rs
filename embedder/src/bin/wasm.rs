use std::{fs::File, io::Read};
use wasmer::{imports, Instance, Module, Store};
// use wasmer_wasix::WasiEnv;

static PLUGIN_PATH: &'static str = "./target/wasm32-unknown-unknown/release/wasm_plugin.wasm";

fn main(){
    let mut f = File::open(PLUGIN_PATH).unwrap();
    let mut wasm_plugin = Vec::default();
    f.read_to_end(&mut wasm_plugin).unwrap();

    let mut store = Store::default();
    let module = Module::new(&store, &wasm_plugin).unwrap();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _guard = runtime.enter();

    // let mut wasi_env = WasiEnv::builder("engine").finalize(&mut store).unwrap();
    // let import_object = wasi_env.import_object(&mut store, &module).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    // wasi_env.initialize(&mut store, instance.clone()).unwrap();

    let start = instance.exports.get_typed_function::<u8, u8>(&mut store, "add").unwrap();
    let result = start.call(&mut store, 2).unwrap();
    assert_eq!(result, 2);

    let result = start.call(&mut store, 3).unwrap();
    assert_eq!(result, 5);
}
