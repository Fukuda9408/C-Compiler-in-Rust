#!/bin/bash
assert() {
    expected="$2"
    input="$1"

    ./target/debug/c99 "$input" > tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "#input => $expected expected, but got $actual"
        exit 1
    fi
}

assert "./test/test1.txt" 3
assert "./test/test2.txt" 18
assert "./test/test3.txt" 3
assert "./test/test4.txt" 4
assert "./test/test5.txt" 15
assert "./test/test6.txt" 1
assert "./test/test7.txt" 0
assert "./test/test8.txt" 0
assert "./test/test9.txt" 1
assert "./test/test10.txt" 1
assert "./test/test11.txt" 0
assert "./test/test12.txt" 0
assert "./test/test13.txt" 1

assert "./test/test14.txt" 1
assert "./test/test15.txt" 0
assert "./test/test16.txt" 0
assert "./test/test17.txt" 1

assert "./test/test18.txt" 1
assert "./test/test19.txt" 0
assert "./test/test20.txt" 0
assert "./test/test21.txt" 1

assert "./test/test22.txt" 2
assert "./test/test23.txt" 6

assert "./test/test24.txt" 3
assert "./test/test25.txt" 0
assert "./test/test26.txt" 1
assert "./test/test27.txt" 4
assert "./test/test28.txt" 6
assert "./test/test29.txt" 3
# 二個目のfo./test/test2.txtrループでは初期化処理が働かないため、b=3に一個目のループで変化した後は、aの値を変えようと二個目のループの中身は処理されないため図ずっとc=3
assert "./test/test30.txt" 6

assert "./test/test31.txt" 0
assert "./test/test32.txt" 2
assert "./test/test33.txt" 1
assert "./test/test34.txt" 2
assert "./test/test35.txt" 6
assert "./test/test36.txt" 10
assert "./test/test37.txt" 48

assert "./test/test38.txt" 3

assert "./test/test39.txt" 10
echo OK
