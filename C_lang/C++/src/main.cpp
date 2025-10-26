#include <iostream>

int main() {
    std::cout << "Hello, C++!" << std::endl;
    return 0;
}


/*
PS E:\RustCode\C_lang\C++> make
g++ -Wall -O2 -c src/main.cpp -o src/main.o
g++ src/main.o -o hello_cpp.exe

PS E:\RustCode\C_lang\C++> .\hello_cpp.exe
Hello, C++!
*/