package main

import (
	"bufio"
	"fmt"
	"os"
)

func Ord(c rune) int {
	return int(c) - 'a'
}

func main() {
	in := bufio.NewReader(os.Stdin)

	var s string
	fmt.Fscan(in, &s)

	n := len(s)

	cnt := make([]int, 26)

	for _, c := range s {
		cnt[Ord(c)] += 1
	}

	// 答えが変わらない交換を除く
	isSame := false
	ans := n * n

	for _, k := range cnt {
		if k > 1 {
			isSame = true
		}

		ans -= k * k
	}

	ans /= 2

	if isSame {
		ans += 1
	}

	fmt.Println(ans)
}
