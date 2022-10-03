use libc::c_long;

#[no_mangle]
pub extern "C" fn sum(x: c_long, y: c_long) -> c_long {
    x + y
}

#[no_mangle]
pub extern "C" fn sum_long_running(x: c_long, y: c_long) -> c_long {
    x + y
}
