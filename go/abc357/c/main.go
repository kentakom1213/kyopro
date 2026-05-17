package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Sierpinski(n int) []string {
	if n == 0 {
		return []string{"#"}
	}
	prev := Sierpinski(n - 1)
	p := len(prev)
	res := make([]string, 3*p)

	for r := 0; r < 3; r += 1 {
		for c := 0; c < 3; c += 1 {
			if r == 1 && c == 1 {
				// '.' で埋める
				for k := 0; k < p; k += 1 {
					res[p*r+k] += strings.Repeat(".", p)
				}
			} else {
				for k, row := range prev {
					res[p*r+k] += row
				}
			}
		}
	}

	return res
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var n int
	fmt.Fscan(in, &n)

	ans := Sierpinski(n)

	for _, s := range ans {
		fmt.Fprintln(out, s)
	}
}
