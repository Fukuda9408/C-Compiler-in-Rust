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

assert " 1 + 2" 3
assert " 1 + 3 * 4 + 5" 18
assert " 1 + +2" 3
assert "1 - -3 " 4
assert "1 - -(4 + 5 * 2)" 15

echo OK
