#!/bin/bash

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


UNMATCHED1=$(./target/debug/brainfuck ./examples/unmatched1.bf 2>&1)
if [ "$UNMATCHED1" != "ERROR: unmatched bracets" ]; then
	echo fail: unmatched1
	exit
fi

UNMATCHED2=$(./target/debug/brainfuck ./examples/unmatched2.bf 2>&1)
if [ "$UNMATCHED2" != "ERROR: unmatched bracets" ]; then
	echo fail: unmatched1
	exit
fi

echo success
