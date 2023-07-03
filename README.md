generate.js 用于生成数据

Memory:
valgrind --tool=memcheck ./target/release/rust
valgrind --tool=memcheck ./sort
CPU:
valgrind --tool=callgrind ./target/release/rust
valgrind --tool=callgrind ./sort

gprof2dot --format=callgrind --output=out.dot ./callgrind.out.521
dot -Tpng rust.svg -o graph.png
----


CPU: Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz   3.19 GHz
RAM: 16.0 GB

RESULT
V language
```
==507==
==507== HEAP SUMMARY:
==507==     in use at exit: 2,992 bytes in 11 blocks
==507==   total heap usage: 505,129 allocs, 505,118 frees, 32,341,222 bytes allocated
==507==
==507== LEAK SUMMARY:
==507==    definitely lost: 0 bytes in 0 blocks
==507==    indirectly lost: 0 bytes in 0 blocks
==507==      possibly lost: 2,992 bytes in 11 blocks
==507==    still reachable: 0 bytes in 0 blocks
==507==         suppressed: 0 bytes in 0 blocks
==507== Rerun with --leak-check=full to see details of leaked memory
==507==
==507== Use --track-origins=yes to see where uninitialised values come from
==507== For lists of detected and suppressed errors, rerun with: -s
==507== ERROR SUMMARY: 1990 errors from 102 contexts (suppressed: 0 from 0)
```

Rust
```
==504== Memcheck, a memory error detector
==504== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==504== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==504== Command: ./target/release/rust
==504==
In file ../data.json
==504==
==504== HEAP SUMMARY:
==504==     in use at exit: 0 bytes in 0 blocks
==504==   total heap usage: 1,159 allocs, 1,159 frees, 21,370,367 bytes allocated
==504==
==504== All heap blocks were freed -- no leaks are possible
==504==
==504== For lists of detected and suppressed errors, rerun with: -s
==504== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```