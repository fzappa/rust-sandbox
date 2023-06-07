#include "cpp_test2_func.h"

extern "C" {
    int cpp_test2_func(int x) {
        return x * 2;
    }
}

