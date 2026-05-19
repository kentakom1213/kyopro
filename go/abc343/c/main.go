package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var IN *bufio.Reader
var OUT *bufio.Writer

func InitIO() {
	IN = bufio.NewReader(os.Stdin)
	OUT = bufio.NewWriter(os.Stdout)
}

func ReadVal[T any]() T {
	var v T
	fmt.Fscan(IN, &v)
	return v
}

func ReadVec[T any](n int) []T {
	arr := make([]T, n)
	for i := range n {
		arr[i] = ReadVal[T]()
	}
	return arr
}

func ReadVec2[T any](n, m int) [][]T {
	arr := make([][]T, 0, n)
	for i := 0; i < n; i += 1 {
		line := ReadVec[T](m)
		arr = append(arr, line)
	}
	return arr
}

func IsPalindrome(n int) bool {
	s := strconv.Itoa(n)
	k := len(s)
	for i := 0; i <= k/2; i += 1 {
		if s[i] != s[k-i-1] {
			return false
		}
	}
	return true
}

func main() {
	InitIO()
	defer OUT.Flush()

	N := ReadVal[int]()

	ans := 1

	for i := 1; i*i*i <= N; i += 1 {
		n := i * i * i
		if IsPalindrome(n) {
			ans = n
		}
	}

	fmt.Fprintln(OUT, ans)
}
