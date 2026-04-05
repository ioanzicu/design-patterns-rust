## Compile the C library

- Linux/Mac OS

    ```bash
    gcc -c my_c_lib.c -o my_c_lib.o
    ```

    ```bash
    ar rcs limby_c_lib.a my_c_lib.o
    ```

It will create `libmy_c_lib.a` or `my_c_lib.lib` in the current directory.