package main

import (
	"fmt"
	"math"
)

func abs(x int) int {
	return int(math.Abs(float64(x)))
}

func max(x, y int) int {
	return int(math.Max(float64(x), float64(y)))
}

func min(x, y int) int {
	return int(math.Min(float64(x), float64(y)))
}

func min_n(nums ...int) int {
	size := len(nums)
	if size == 0 {
		panic("func min() requires at least 1 argument.")
	}
	res := nums[0]
	for i := 1; i < size; i++ {
		res = min(res, nums[i])
	}
	return res
}

func main() {
	x, y := 2, 5
	mx := int(math.Max(float64(x), float64(y)))
	fmt.Println(mx)

	fmt.Println(max(20, 50))
	fmt.Println(min(20, 50))
	fmt.Println(min_n(20, 50, 10, 40))
	// fmt.Println(min())
}
