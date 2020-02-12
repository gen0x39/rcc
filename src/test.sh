#!/bin/bash

rcc="../target/debug/rcc"

try() {
  expected="$1"
  input="$2"

  ${rcc} "$input" > tmp.s
  gcc -static -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

cargo build

try 0 0
try 42 42
try 1 "2+6-7"
try 41 " 12 + 34 - 5 "

rm tmp*

echo OK