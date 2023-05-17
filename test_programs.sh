#!/bin/bash

set -e

cargo build

HELLO=$(./target/debug/brainfuck ./examples/helloworld.bf)
if [ "$HELLO" != "Hello World!" ]; then
	echo fail: hello world
	exit
fi

OBSCURE=$(./target/debug/brainfuck ./examples/obscure_problems.bf)
if [ "$OBSCURE" != "H" ]; then
	echo fail: obscure
	exit
fi

ARRAY=$(./target/debug/brainfuck ./examples/array_size.bf)
if [ "$ARRAY" != "#" ]; then
	echo fail: array
	exit
fi

echo success
