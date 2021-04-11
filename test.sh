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
assert "b = 0; for(a = 0; a < 4; a = a + 1) b = b + a;return b;" 6
assert "a=0;b=0;c=0;for(;a<4;a = a+1) for(;b<4;b = b+1) c = a+b;return c;" 3
# 二個目のforループでは初期化処理が働かないため、b=3に一個目のループで変化した後は、aの値を変えようと二個目のループの中身は処理されないため図ずっとc=3
assert "c=0;for(a=0;a<4;a = a+1) for(b=0;b<4;b = b+1) c = a+b;return c;" 6

assert "a=1;c=1;if(c >0) {a=0;} return a;" 0
assert "a=1;c=1;if(c >2) {a=0;} else {a = 2;} return a;" 2
assert "a=1;c=1;if(c >2) {a=0;} return a;" 1
assert "a=1;c=1;if(c >2) {a=0;} else {a = 2;} return a;" 2
assert "a=0;b=0;while(a>4){b = b + a; a = a + 1;}return b;" 6
assert "b=0;for(a=0; a < 5; a = a + 1) {b = b + a;}" 10
assert "sum=0;for(a=0; a < 4; a = a + 1) {
            for(b=0;b < 3; b = b + 1) {
                sum = sum + a + b;
            }
        }
        returm sum;" 48

echo OK
