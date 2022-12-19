package main

import (
	"fmt"
	"sort"
)

/*
func abs(x int) int {
	return int(math.Abs(float64(x)))
}

func is_ok(right int) bool {
	return true || false
}

func bisect(ng, ok int) int {
	for abs(ok-ng) > 1 {
		middle := (ok + ng) / 2
		if is_ok(middle) {
			ok = middle
		} else {
			ng = middle
		}
	}
	return ok
}
*/

func main() {
	// standard
	nums := []int{0, 5, 8, 16}
	length := len(nums)
	target := 8
	idx := sort.Search(length, func(i int) bool { return nums[i] >= target })
	if idx < length && nums[idx] == target {
		fmt.Println(idx)
	} else {
		fmt.Println("not found")
	}

	// --------------------------------------------
	nums = []int{4, 5, 6, 7, 0, 1, 2}
	length = len(nums)
	// border
	start := sort.Search(length, func(r int) bool { return nums[0] > nums[r] })
	fmt.Println("start index: ", start)

	target = 6
	// find from {4, 5, 6, 7}
	idx = sort.Search(start, func(i int) bool { return nums[i] >= target })
	if idx < start && nums[idx] == target {
		fmt.Println(idx)
	} else {
		fmt.Println("leftside: not found")
	}

	// find from {0, 1, 2}
	target = 1
	idx = sort.Search(length-start, func(i int) bool { return nums[start+i] >= target })
	if idx < length-start && nums[start+idx] == target {
		fmt.Println(start + idx)
	} else {
		fmt.Println("rightside: not found")
	}
}
