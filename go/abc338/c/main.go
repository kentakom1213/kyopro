package main

import (
	"bufio"
	"cmp"
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

func MaxSlice[T cmp.Ordered](a []T) (T, bool) {
	var zero T

	if len(a) == 0 {
		return zero, false
	}

	res := a[0]
	for _, x := range a[1:] {
		if x > res {
			res = x
		}
	}
	return res, true
}

func MinSlice[T cmp.Ordered](a []T) (T, bool) {
	var zero T

	if len(a) == 0 {
		return zero, false
	}

	res := a[0]
	for _, x := range a[1:] {
		if x < res {
			res = x
		}
	}
	return res, true
}

const INF = 1001001001001001001

func main() {
	defer OUT.Flush()

	n := readVal[int]()
	Q := readVec[int](n)
	A := readVec[int](n)
	B := readVec[int](n)

	maxQ, _ := MaxSlice(Q)

	ans := 0

	for x := range maxQ + 1 {
		y := INF
		for i := range n {
			if Q[i] < A[i]*x {
				y = -INF
			} else if B[i] > 0 {
				y = min(
					y,
					(Q[i]-A[i]*x)/B[i],
				)
			}
		}
		ans = max(ans, x+y)
	}

	print(ans)
}
