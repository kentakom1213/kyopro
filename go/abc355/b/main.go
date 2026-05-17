package main

import (
	"fmt"
	"sort"
)

func MergeArrays(arr1, arr2 []int) []int {
	n, m := len(arr1), len(arr2)
	merged := make([]int, 0, n+m)

	i, j := 0, 0
	for i < n || j < m {
		if j == m || i < n && arr1[i] <= arr2[j] {
			merged = append(merged, arr1[i])
			i += 1
		} else {
			merged = append(merged, arr2[j])
			j += 1
		}
	}

	return merged
}

// arr における val 以上の最小の要素をみつける
func LowerBound(arr []int, val int) int {
	ng, ok := -1, len(arr)
	for ok-ng > 1 {
		mid := (ng + ok) / 2
		if arr[mid] < val {
			ng = mid
		} else {
			ok = mid
		}
	}
	return ok
}

func Contains(arr []int, val int) bool {
	i := LowerBound(arr, val)
	return i < len(arr) && arr[i] == val
}

func main() {
	var n, m int
	fmt.Scan(&n, &m)

	A := make([]int, n)
	for i := range A {
		fmt.Scan(&A[i])
	}
	sort.Ints(A)

	B := make([]int, m)
	for i := range B {
		fmt.Scan(&B[i])
	}
	sort.Ints(B)

	C := MergeArrays(A, B)

	contains_l := Contains(A, C[0])

	for i := range n + m - 1 {
		contains_r := Contains(A, C[i+1])

		if contains_l && contains_r {
			fmt.Println("Yes")
			return
		}

		contains_l = contains_r
	}

	fmt.Println("No")
}
