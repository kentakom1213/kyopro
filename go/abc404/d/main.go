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

func initVec2[T any](n, m int) [][]T {
	arr := make([][]T, n)
	for i := range n {
		arr[i] = make([]T, m)
	}
	return arr
}

func PowInt(a, e int) int {
	res := 1
	for e > 0 {
		if e%2 == 1 {
			res *= a
		}
		a *= a
		e /= 2
	}
	return res
}

const INF int = 1001001001001001001

func main() {
	defer OUT.Flush()

	N := readVal[int]()
	M := readVal[int]()
	C := readVec[int](N)

	A := initVec2[int](N, M)

	for i := range M {
		k := readVal[int]()
		for range k {
			j := readVal[int]() - 1
			A[j][i] = 1
		}
	}

	ans := INF

	for S := range PowInt(3, N) {
		cost := 0
		cnt := make([]int, M)
		for i := range N {
			use := S % 3
			// fmt.Printf("%d,", use)

			cost += C[i] * use
			for j, a := range A[i] {
				cnt[j] += a * use
			}
			S /= 3
		}
		// fmt.Println()

		isOk := true
		for _, v := range cnt {
			if v <= 1 {
				isOk = false
				break
			}
		}

		// fmt.Println(cnt, isOk, cost)

		if isOk && ans > cost {
			ans = cost
		}

	}

	print(ans)
}
