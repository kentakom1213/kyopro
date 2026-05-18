package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	if a+b == 0 {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}
