package main

import (
	"bufio"
	"fmt"
	"os"
)

func ReadVec[T any](in *bufio.Reader, n int) []T {
	arr := make([]T, n)
	for i := range n {
		fmt.Fscan(in, &arr[i])
	}
	return arr
}

func ReadVec2[T any](in *bufio.Reader, n, m int) [][]T {
	arr := make([][]T, 0, n)
	for i := 0; i < n; i += 1 {
		line := ReadVec[T](in, m)
		arr = append(arr, line)
	}
	return arr
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var n int
	fmt.Fscan(in, &n)

	A := ReadVec2[int](in, n, n)

	for i := range n {
		for j := range n {
			if A[i][j] == 1 {
				fmt.Fprintf(out, "%d ", j+1)
			}
		}
		fmt.Fprintln(out)
	}
}
