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

func NewVecWith[T any](n int, initVal T) []T {
	arr := make([]T, n)
	for i := range n {
		arr[i] = initVal
	}
	return arr
}

const INF int = 1001001001001001001

func main() {
	defer OUT.Flush()

	N := readVal[int]()
	C := make([]int, N)
	A := make([]int, N)
	for i := range N - 1 {
		c := readVal[int]()
		C[i+1] = c
	}

	last1 := 0
	for i := range N - 1 {
		a := readVal[int]()
		A[i+1] = a
		if a > 0 {
			last1 = i + 1
		}
	}

	// fmt.Println(C)
	// fmt.Println(A)

	// dp[i] := i+1 番目以降の箱の要素をすべて i 番目の箱に移動するときのコストの最小値
	dp := NewVecWith(N, INF)
	dp[last1] = 0

	for i := N - 1; i >= 0; i -= 1 {
		for j := i + 1; j <= last1; j += 1 {
			if j-i <= C[j] {
				// 直接行ける場合
				dp[i] = min(dp[i], dp[j]+1)
			}
		}

		if A[i] > 0 {
			last1 = i
		}

		// fmt.Println(dp)
	}

	print(dp[0])
}
