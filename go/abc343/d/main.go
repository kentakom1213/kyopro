package main

import (
	"bufio"
	"fmt"
	"os"
)

var IN *bufio.Reader
var OUT *bufio.Writer

func InitIO() {
	IN = bufio.NewReader(os.Stdin)
	OUT = bufio.NewWriter(os.Stdout)
}

func ReadVal[T any]() T {
	var v T
	fmt.Fscan(IN, &v)
	return v
}

func ReadVec[T any](n int) []T {
	arr := make([]T, n)
	for i := range n {
		arr[i] = ReadVal[T]()
	}
	return arr
}

func ReadVec2[T any](n, m int) [][]T {
	arr := make([][]T, 0, n)
	for i := 0; i < n; i += 1 {
		line := ReadVec[T](m)
		arr = append(arr, line)
	}
	return arr
}

func main() {
	InitIO()
	defer OUT.Flush()

	n := ReadVal[int]()
	t := ReadVal[int]()
	AB := ReadVec2[int](t, 2)

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

		// fmt.Println(points)
		// fmt.Println(s)

		fmt.Fprintln(OUT, len(s))
	}
}
