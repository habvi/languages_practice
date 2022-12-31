package main

import (
	"fmt"
	"strings"
)

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

	for k, v := range counts {
		fmt.Println(k, v)
	}
}

func main() {
	type_struct()
	type_map()
}
