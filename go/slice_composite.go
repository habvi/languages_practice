package main

import "fmt"

// pointer, len, cap
func type_slice() {
	// array
	var ns1 [3]int
	ns1[1] = 2
	fmt.Println(ns1)

	var ns2 = [3]int{2, 4, 8}
	fmt.Println(ns2)

	ns3 := [...]int{10, 15, 4: 25}
	fmt.Println(len(ns3), ns3, ns3[1:4])

	// slice
	var ns4 []int            // nil
	ns4 = make([]int, 3, 10) // length, capacity
	fmt.Println(ns4)
	// almost same
	// var array [10]int
	// ns4 := array[:3]

	var ns5 = []int{2, 3, 4} // initialize with a slice literal. list created auto
	fmt.Println(ns5)
	// almost same
	// var array [...]{2, 3, 4}
	// ns5 := array[:]

	ns6 := []int{2: 5, 5: 10}
	fmt.Println(ns6)
	ns6 = append(ns6, 33)
	fmt.Println(ns6)
	ns6 = append(ns6, 44, 55)
	fmt.Println(ns6)
	println(len(ns6), cap(ns6))
}

func make_1d() {
	var vec1 [4]int
	fmt.Println(vec1)

	vec2 := [4]int{}
	fmt.Println(vec2)

	vec3 := make([]int, 4)
	fmt.Println(vec3)
}

func make_2d() {
	// const
	var vec1 [3][5]int
	fmt.Println(vec1)

	// val
	h, w := 3, 5
	vec2 := make([][]int, h)
	for i := 0; i < h; i++ {
		vec2[i] = make([]int, w)
	}
	fmt.Println(vec2)
}

func insert_i_th() {
	vec := []int{1, 2, 3, 4, 5, 6}
	i := 2

	vec = append(vec[:i+1], vec[i:]...)
	fmt.Println(vec)
	vec[i] = 100
	fmt.Println(vec)
}

func main() {
	type_slice()
	make_1d()
	make_2d()
	insert_i_th()
}
