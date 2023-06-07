extern "C" {
    fn cpp_test1_func();
}

fn main() {
    unsafe {
        cpp_test1_func();
    }
}

