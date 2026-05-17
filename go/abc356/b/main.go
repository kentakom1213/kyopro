package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var n, m int
	fmt.Fscan(in, &n, &m)

	A := make([]int, m)
	for i := range A {
		fmt.Fscan(in, &A[i])
	}

	X := make([][]int, n)
	for r := range n {
		x := make([]int, m)
		for c := range m {
			fmt.Fscan(in, &x[c])
		}
		X[r] = x
	}

	sum := make([]int, m)
	for i := range n {
		for j := range m {
			sum[j] += X[i][j]
		}
	}

	isOk := true

	for i := 0; i < m; i += 1 {
		isOk = isOk && (A[i] <= sum[i])
	}

	if isOk {
		fmt.Fprintln(out, "Yes")
	} else {
		fmt.Fprintln(out, "No")
	}
}
