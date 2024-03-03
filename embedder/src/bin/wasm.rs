use std::{fs::File, io::Read};
use protobuf::Message;
use wasmer::{Instance, Module, Store};
use wasmer_wasix::WasiEnv;
use embedder::values::EchoIO;

static PLUGIN_PATH: &'static str = "./target/wasm32-wasi/release/wasm_plugin.wasm";

#[tokio::main]
async fn main() {
    let mut f = File::open(PLUGIN_PATH).unwrap();
    let mut wasm_plugin = Vec::default();
    f.read_to_end(&mut wasm_plugin).unwrap();

    let mut store = Store::default();
    let module = Module::new(&store, &wasm_plugin).unwrap();

    let mut wasi_env = WasiEnv::builder("engine").finalize(&mut store).unwrap();
    let import_object = wasi_env.import_object(&mut store, &module).unwrap();
    // let import_object = imports! {};
    
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    wasi_env.initialize(&mut store, instance.clone()).unwrap();

    let add = instance
        .exports
        .get_typed_function::<u8, u8>(&mut store, "add")
        .unwrap();

    let result = add.call(&mut store, 2).unwrap();
    assert_eq!(result, 2);

    let result = add.call(&mut store, 3).unwrap();
    assert_eq!(result, 5);

    let mut input = EchoIO::new();
    input.message = "hello world".to_string();

    let input_bytes = input.write_to_bytes().unwrap();

    let heap_start = 0x110000;
    let memory = instance.exports.get_memory("memory").unwrap();
    let pages = (input_bytes.len() / wasmer::WASM_PAGE_SIZE) + 1;
    memory.grow(&mut store, pages as u32).unwrap();

    let view = memory.view(&store);
    view.write(heap_start as u64, &input_bytes).unwrap();

    let echo = instance
        .exports
        .get_typed_function::<i32, i32>(&mut store, "echo")
        .unwrap();

    let _result = echo.call(&mut store, heap_start).unwrap();
}
