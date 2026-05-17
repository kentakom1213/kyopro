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
	fmt.Fscan(in, &n)
	fmt.Fscan(in, &m)

	H := make([]int, n)

	for i := 0; i < n; i += 1 {
		fmt.Fscan(in, &H[i])
	}

	ans := 0

	for i := 0; i < n; i += 1 {
		if m < H[i] {
			break
		}
		ans += 1
		m -= H[i]
	}

	fmt.Fprintln(out, ans)
}
