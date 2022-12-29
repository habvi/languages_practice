package main

import "fmt"

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

func main() {
	make_1d()
	make_2d()
}
