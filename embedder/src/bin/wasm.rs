use byteorder::{LittleEndian, ReadBytesExt};
use proto::values::EchoIO;
use protobuf::Message;
use std::{fs::File, io::Read};
use wasmer::{Instance, Module, Store, WasmSlice};
use wasmer_wasix::WasiEnv;

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

    let input = EchoIO {
        message: "hello world".to_string(),
        ..Default::default()
    };

    let input_bytes = input.write_to_bytes().unwrap();
    let input_len = (input_bytes.len() as u32).to_le_bytes();
    let input_bytes = [&input_len[..], &input_bytes].concat();

    let heap_start = 0x110000;
    let memory = instance.exports.get_memory("memory").unwrap();
    let pages = (input_bytes.len() / wasmer::WASM_PAGE_SIZE) + 1;
    memory.grow(&mut store, pages as u32).unwrap();

    let view = memory.view(&store);
    view.write(heap_start as u64, &input_bytes).unwrap();

    let echo = instance
        .exports
        .get_typed_function::<u32, u32>(&mut store, "echo")
        .unwrap();

    let pointer = echo.call(&mut store, heap_start).unwrap();
    let view = memory.view(&store);

    let output_len = {
        let bytes = WasmSlice::new(&view, pointer as u64, 4)
            .unwrap()
            .read_to_vec()
            .unwrap();
        bytes.as_slice().read_u32::<LittleEndian>().unwrap()
    };

    let output_bytes = WasmSlice::new(&view, pointer as u64 + 4, output_len as u64)
        .unwrap()
        .read_to_vec()
        .unwrap();
    let output = EchoIO::parse_from_bytes(&output_bytes).unwrap();
    assert_eq!(output.message, input.message);
}
