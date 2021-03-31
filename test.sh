#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/compiler "$input" > tmp.s
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

echo OK
