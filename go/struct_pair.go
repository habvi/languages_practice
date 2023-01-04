package main

import (
	"fmt"
	"math"
	"sort"
)

type Pair struct {
	X int
	Y int
}

type PairVec []Pair

func (v PairVec) Len() int {
	return len(v)
}

func (v PairVec) Less(i, j int) bool {
	if v[i].X == v[j].X {
		return v[i].Y < v[j].Y
	}
	return v[i].X < v[j].X
}

func (v PairVec) Swap(i, j int) {
	v[i], v[j] = v[j], v[i]
}

func min(x, y int) int {
	return int(math.Min(float64(x), float64(y)))
}

func max(x, y int) int {
	return int(math.Max(float64(x), float64(y)))
}
func abs(x int) int {
	return int(math.Abs(float64(x)))
}

func main() {
	p := Pair{5, 6}
	fmt.Println(p, p.X, p.Y)

	var vec PairVec
	vec = append(vec, Pair{2, 8})
	vec = append(vec, Pair{2, 3})
	vec = append(vec, Pair{1, 9})
	fmt.Println(vec)
	sort.Sort(vec)
	fmt.Println(vec)
	sort.Sort(sort.Reverse(vec))
	fmt.Println(vec)
}
