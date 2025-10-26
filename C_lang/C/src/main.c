#include <stdio.h>

int main(void)
{
    printf("Hello, MSYS2 UCRT64 on Windows!\n");
    return 0;
}

/*


PS E:\RustCode\C\C> make
gcc -Wall -O2   -c -o src/main.o src/main.c
gcc src/main.o -o hello.exe

.\hello.exe

*/