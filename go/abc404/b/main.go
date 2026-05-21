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

func stringsToByteArr2D(strs []string) [][]byte {
	n := len(strs)
	arr2 := make([][]byte, n)

	for i, s := range strs {
		for _, c := range s {
			arr2[i] = append(arr2[i], byte(c))
		}
	}

	return arr2
}

// arr2 を左に 90 度回転する
func rotate90(arr2 [][]byte) [][]byte {
	n := len(arr2)
	m := len(arr2[0])
	rot := make([][]byte, m)

	for j := range m {
		for i := range n {
			rot[j] = append(rot[j], arr2[i][m-j-1])
		}
	}

	return rot
}

const INF int = 1001001001001001001

func main() {
	defer OUT.Flush()

	N := readVal[int]()
	S := stringsToByteArr2D(readVec[string](N))
	T := stringsToByteArr2D(readVec[string](N))

	ans := INF

	for _, r := range []int{0, 3, 2, 1} {
		// 距離を計算
		dist := r
		for i := range N {
			for j := range N {
				if S[i][j] != T[i][j] {
					dist += 1
				}
			}
		}

		if ans > dist {
			ans = dist
		}

		S = rotate90(S)
	}

	print(ans)
}
