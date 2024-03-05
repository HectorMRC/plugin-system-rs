use std::sync::Mutex;

static ADD_BASE: Mutex<u8> = Mutex::new(0);

#[no_mangle]
fn add(delta: u8) -> u8 {
    let mut value = ADD_BASE.lock().unwrap();
    *value += delta;
    *value
}
