use std::sync::Mutex;
use proto::values::EchoIO;
use protobuf::Message;

static ADD_BASE: Mutex<u8> = Mutex::new(0);

#[no_mangle]
fn add(delta: u8) -> u8 {
    let mut value = ADD_BASE.lock().unwrap();
    *value += delta;
    *value
}

#[no_mangle]
fn echo(ptr: u32) -> *const u8 {
    let input = unsafe {
         let len = *(ptr as *const u32);
         let bytes = (ptr + 4) as *const u8;
         let slice = core::slice::from_raw_parts(bytes, len as usize);
         EchoIO::parse_from_bytes(slice).unwrap()
    };

    let output = EchoIO{
        message: input.message,
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}
