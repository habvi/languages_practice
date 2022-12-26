package main

import (
	"fmt"
	"sort"
	"strings"
)

func sort_string(s string) string {
	tmp := strings.Split(s, "")
	sort.Strings(tmp)
	sorted_s := strings.Join(tmp, "")
	return sorted_s
}

func main() {
	s := "cdaeb"
	fmt.Println(sort_string(s))
}
