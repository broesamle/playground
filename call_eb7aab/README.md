`call_eb7aab`
=============

Minimal example to compile and link C to rust.


the rust library `return_eb7aab`
--------------------------------

+ the library `libreturn_eb7aab.a`
+ the header file `return_eb7aab.h`


build
-----

compile:  
`gcc -c call_eb7aab.c -I ../return_eb7aab/c_include/`

link:  
`gcc call_eb7aab.o -L ../return_eb7aab/target/debug/ -l return_eb7aab`
