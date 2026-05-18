package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)

	isOk := true

	if !strings.HasPrefix(s, "<") {
		isOk = false
	}

	if !strings.HasSuffix(s, ">") {
		isOk = false
	}

	for i := 1; i < len(s)-1; i += 1 {
		if s[i] != '=' {
			isOk = false
		}
	}

	if isOk {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
