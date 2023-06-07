# 02-rust_cpp_with_parameter

```sh
cd lib
mkdir build
cd build
cmake ..
make

cd ../..
# Go to 02-rust_cpp_with_parameter folder
export LD_LIBRARY_PATH=$PWD/lib/build:$LD_LIBRARY_PATH
cargo run

```
