package main

import "fmt"

func FloorDiv(a, b int) int {
	q := a / b
	r := a % b
	if r != 0 && a < 0 {
		q -= 1
	}
	return q
}

func main() {
	var x int
	fmt.Scan(&x)

	ans := FloorDiv(x+9, 10)

	fmt.Println(ans)
}
