package main

import (
	"bufio"
	"fmt"
	"os"
)

var IN = bufio.NewReader(os.Stdin)
var OUT = bufio.NewWriter(os.Stdout)

func readVal[T any]() T {
	var v T
	fmt.Fscan(IN, &v)
	return v
}

func readVec[T any](n int) []T {
	arr := make([]T, n)
	for i := range n {
		arr[i] = readVal[T]()
	}
	return arr
}

func readVec2[T any](n, m int) [][]T {
	arr := make([][]T, 0, n)
	for i := 0; i < n; i += 1 {
		line := readVec[T](m)
		arr = append(arr, line)
	}
	return arr
}

func print(vals ...interface{}) {
	fmt.Fprintln(OUT, vals...)
}

const ASCII_LOWERCASES string = "abcdefghijklmnopqrstuvwxyz"

func main() {
	defer OUT.Flush()

	S := readVal[string]()
	N := len(S)

	count := make(map[string]int)

	for i := range N {
		c := S[i:i+1]
		count[c] += 1
	}

	for i := range 26 {
		c := ASCII_LOWERCASES[i:i+1]
		if count[c] == 0 {
			print(c)
			return
		}
	}
}
