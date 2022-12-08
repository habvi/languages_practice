package main

import (
	"fmt"
	"strings"
)

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

func type_struct() {
	var p1 struct {
		name string
		age  int
	}
	p1.name = "Bob"
	p1.age++
	fmt.Println(p1)

	p2 := struct {
		name string
		age  int
	}{
		name: "Alice",
		age:  10,
	}
	fmt.Println(p2)
}

func type_map() {
	var m1 map[string]int
	// m1["ab"] = 10 // error
	if m1 == nil {
		fmt.Println("nil!")
	}

	m2 := map[string]int{"ab": 10, "cd": 20}
	fmt.Println(m2)

	m3 := make(map[string]int)
	m3["ab"] = 10
	fmt.Println(m3)

	v, ok := m3["ab"]
	fmt.Println(v, ok)

	delete(m3, "ab")

	m4 := map[string]int{} // same m3
	fmt.Println(m4)

	m5 := make(map[string]int, 10) // cap
	fmt.Println(m5)

	counts := map[string]int{}
	str := "a b b c b a a a"
	for _, s := range strings.Split(str, " ") {
		counts[s]++
	}
	fmt.Println(counts)
}

func main() {
	type_slice()
	type_struct()
	type_map()
}
