package main

import (
	"fmt"
	"strconv"
)

func main() {
	s := "12345"
	d, err := strconv.Atoi(s)
	fmt.Println(d, err)

	d = 12345
	s = strconv.Itoa(d)
	fmt.Println(s)
}
