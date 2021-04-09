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

assert " 1 + 2;" 3
assert " 1 + 3 * 4 + 5;" 18
assert " 1 + +2;" 3
assert "1 - -3 ;" 4
assert "1 - -(4 + 5 * 2);" 15
assert "1 + 5 > 5;" 1
assert "1 + 5 > 7;" 0
assert "1 + 5 < 5;" 0
assert "1 + 5 < 7;" 1
assert "1 + 5 >= 6;" 1
assert "1 + 5 >= 7;" 0
assert "1 + 5 <= 5;" 0
assert "1 + 5 <= 6;" 1

assert "1 + 5 == 6;" 1
assert "1 + 5 == 5;" 0
assert "1 + 5 != 6;" 0
assert "1 + 5 != 5;" 1

assert "5 > 6 == 5 > 6;" 1
assert "5 > 6 == 5 < 6;" 0
assert "5 > 6 != 5 > 6;" 0
assert "5 > 6 != 5 < 6;" 1

assert "a =1;b =1; a = a+b;" 2
assert " a =3; b = a; c = b + a;" 6

assert "a=1; b = 2; return a + b;" 3
assert "a=1; if(a==1) b = 0;" 0
assert "a = 1; if(a == 0) b = 0; else b = 1;" 1
assert "a = 1; while(a < 4) a = a + 1; " 4
assert "b = 0; for(a = 0; a < 4; a = a + 1) b = b + a;" 6

echo OK
