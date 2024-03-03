use std::sync::Mutex;

static ADD_BASE: Mutex<u8> = Mutex::new(0);

#[no_mangle]
pub extern "C" fn add(delta: u8) -> u8 {
    let mut value = ADD_BASE.lock().unwrap();
    *value += delta;
    *value
}

#[no_mangle]
pub extern "C" fn echo(ptr: i32) -> Box<()> {
    let _msg = unsafe {
         let len = *(ptr as *const u32);
         let bytes = (ptr + 4) as *const u8;
         let slice = core::slice::from_raw_parts(bytes, len as usize);
         core::str::from_utf8_unchecked(slice)
    };

    Box::new(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2);
        assert_eq!(result, 2);

        let result = add(3);
        assert_eq!(result, 5);
    }
}
