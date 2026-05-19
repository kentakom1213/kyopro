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

func main() {
	defer OUT.Flush()

	n := readVal[int]()
	t := readVal[int]()
	AB := readVec2[int](t, 2)

	points := make([]int, n)
	s := map[int]int{
		0: n,
	}

	for _, ab := range AB {
		a, b := ab[0]-1, ab[1]

		s[points[a]] -= 1
		if s[points[a]] == 0 {
			delete(s, points[a])
		}

		points[a] += b
		s[points[a]] += 1

		print(len(s))
	}
}
