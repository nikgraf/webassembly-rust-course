extern {
    fn appendNumberToBody(x: u32);
    fn alert(x: u32);
}

#[no_mangle]
pub extern fn run() {
    unsafe {
        alert(4);
        appendNumberToBody(42);
    }
}
