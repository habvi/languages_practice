package main

import "fmt"

func main() {
	set := make(map[string]struct{})
	set["ab"] = struct{}{}
	set["cde"] = struct{}{}
	fmt.Println(set)

	v, ok := set["cde"]
	fmt.Println(v, ok)
	_, ok = set["zzzz"]
	fmt.Println(ok, set)

	for _, v := range []string{"fg", "hij"} {
		set[v] = struct{}{}
	}
	fmt.Println(set)
}
