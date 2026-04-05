#include <stdio.h>

int multiply(int a, int b) {
    return a * b;
}

void greet_person(const char* name) {
    if (name != NULL) {
        printf("[C Library] Hello, %s!\n");
    } else {
        printf("[C Library] Hello, (null name provided)!\n");
    }
}
