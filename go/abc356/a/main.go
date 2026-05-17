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

	var n, l, r int
	fmt.Fscan(in, &n)
	fmt.Fscan(in, &l)
	fmt.Fscan(in, &r)

	for i := 1; i < l; i += 1 {
		fmt.Fprintln(out, i)
	}

	for i := r; i >= l; i -= 1 {
		fmt.Fprintln(out, i)
	}

	for i := r + 1; i <= n; i += 1 {
		fmt.Fprintln(out, i)
	}
}
