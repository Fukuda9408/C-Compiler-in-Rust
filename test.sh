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
assert "1 + 5 > 5" 1
assert "1 + 5 > 7" 0
assert "1 + 5 < 5" 0
assert "1 + 5 < 7" 1
assert "1 + 5 >= 6" 1
assert "1 + 5 >= 7" 0
assert "1 + 5 <= 5" 0
assert "1 + 5 <= 6" 1

assert "1 + 5 == 6" 1
assert "1 + 5 == 5" 0
assert "1 + 5 != 6" 0
assert "1 + 5 != 5" 1

assert "5 > 6 == 5 > 6" 1
assert "5 > 6 == 5 < 6" 0
assert "5 > 6 != 5 > 6" 0
assert "5 > 6 != 5 < 6" 1

echo OK
