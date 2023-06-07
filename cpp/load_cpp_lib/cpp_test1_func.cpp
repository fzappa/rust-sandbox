#include <iostream>

//g++ -shared -fPIC -o libmycpplib.so cpp_test1_func.cpp

extern "C" {
    void cpp_test1_func() {
        std::cout << "Calling the C++ function from Rust" << std::endl;
    }
}

