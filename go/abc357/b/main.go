package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var s string
	fmt.Fscan(in, &s)

	up, lo := 0, 0

	for _, c := range s {
		if unicode.IsUpper(c) {
			up += 1
		} else {
			lo += 1
		}
	}

	var ans string

	if up > lo {
		ans = strings.ToUpper(s)
	} else {
		ans = strings.ToLower(s)
	}

	fmt.Fprintln(out, ans)
}
