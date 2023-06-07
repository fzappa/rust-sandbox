fn main() {
    println!("cargo:rustc-link-search=native=lib/build");
    println!("cargo:rustc-link-lib=dylib=mycpplib");
}

