extern crate libc;
use libc::c_int;

extern {
    fn cpp_test2_func(x: c_int) -> c_int;
}

fn main() {
    let x: c_int = 10;
    unsafe {
        let res = cpp_test2_func(x);
        println!("Output: {}", res);
    }
}

