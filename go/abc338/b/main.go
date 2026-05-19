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

func print(vals ...interface{}) {
	fmt.Fprintln(OUT, vals...)
}

func ord(c rune) int {
	return int(c) - 'a'
}

func chr(n int) string {
	return string('a' + n)
}

func main() {
	defer OUT.Flush()

	S := readVal[string]()

	cnt := make([]int, 26)

	for _, c := range S {
		cnt[ord(c)] += 1
	}

	ans := 0
	val := 0

	for i := range 26 {
		if cnt[i] > val {
			ans = i
			val = cnt[i]
		}
	}

	print(chr(ans))
}
