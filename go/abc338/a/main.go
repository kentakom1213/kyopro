package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
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

func main() {
	defer OUT.Flush()

	S := readVal[string]()

	pref := S[:1]
	tail := S[1:]

	T := strings.ToUpper(pref) + strings.ToLower(tail)

	if S == T {
		print("Yes")
	} else {
		print("No")
	}
}
