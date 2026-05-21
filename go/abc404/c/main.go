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

func readVec[T any](n int) []T {
	arr := make([]T, n)
	for i := range n {
		arr[i] = readVal[T]()
	}
	return arr
}

func readVec2[T any](n, m int) [][]T {
	arr := make([][]T, 0, n)
	for i := 0; i < n; i += 1 {
		line := readVec[T](m)
		arr = append(arr, line)
	}
	return arr
}

func print(vals ...interface{}) {
	fmt.Fprintln(OUT, vals...)
}

// ========== DSU ==========

type DSU struct {
	n   int
	par []int
	siz []int
}

func NewDSU(n int) *DSU {
	par := make([]int, n)
	siz := make([]int, n)

	for i := range n {
		par[i] = i
		siz[i] = 1
	}

	return &DSU{
		n:   n,
		par: par,
		siz: siz,
	}
}

func (d *DSU) Leader(x int) int {
	r := d.par[x]
	for r != d.par[r] {
		r = d.par[r]
	}
	// 経路圧縮
	for x != r {
		p := d.par[x]
		d.par[x] = r
		x = p
	}
	return r
}

func (d *DSU) Unite(x, y int) int {
	rx := d.Leader(x)
	ry := d.Leader(y)
	if rx == ry {
		return rx
	}
	var r, c int
	if d.siz[rx] > d.siz[ry] {
		r, c = rx, ry
	} else {
		r, c = ry, rx
	}
	d.par[c] = r
	d.siz[r] += d.siz[c]
	return r
}

func (d *DSU) IsSame(x, y int) bool {
	return d.Leader(x) == d.Leader(y)
}

func (d *DSU) Size(x int) int {
	r := d.Leader(x)
	return d.siz[r]
}

func main() {
	defer OUT.Flush()

	N := readVal[int]()
	M := readVal[int]()
	AB := readVec2[int](M, 2)

	if N != M {
		print("No")
		return
	}

	G := make([][]int, N)
	dsu := NewDSU(N)

	for _, e := range AB {
		u, v := e[0]-1, e[1]-1
		G[u] = append(G[u], v)
		G[v] = append(G[v], u)
		dsu.Unite(u, v)
	}

	// 次数を検証
	isDegreeOk := true
	isSpanning := true

	for u := range N {
		if len(G[u]) != 2 {
			isDegreeOk = false
		}
		if !dsu.IsSame(0, u) {
			isSpanning = false
		}
	}

	if !isDegreeOk || !isSpanning {
		print("No")
		return
	}

	print("Yes")
}
