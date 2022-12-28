package main

import (
	"fmt"
	"sort"
)

func sort_1d(v []int) {
	sort.Sort(sort.IntSlice(v))
	fmt.Println(v)

	sort.Sort(sort.Reverse(sort.IntSlice(v)))
	fmt.Println(v)

	sort.Slice(v, func(i, j int) bool { return v[i] < v[j] })
	fmt.Println(v)
}

func sort_2d(v []int) {
	size := len(v)
	vec := [][2]int{}

	for i := 0; i < size; i++ {
		vec = append(vec, [2]int{v[i], i})
	}
	sort.Slice(vec, func(i, j int) bool { return vec[i][0] > vec[j][0] })
	fmt.Println(vec)

	sort.Slice(vec, func(i, j int) bool { return vec[i][0] < vec[j][0] })
	fmt.Println(vec)
}

func main() {
	v := []int{1, 8, 6, 2, 5, 4, 8, 3, 7}
	sort_1d(v)
	sort_2d(v)
}
