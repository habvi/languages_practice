package main

import "fmt"

func main() {
	if a := 20; a < 10 {
		fmt.Println("if", a)
	} else {
		fmt.Println("else", a)
	}
	// fmt.Println("else", a) can't use a

	a := 10
	switch a {
	case 5, 10:
		fmt.Println("5 or 10")
	default:
		fmt.Println("default")
	}

	switch {
	case a%5 == 0:
		fmt.Println("divisible by 5")
	}
}
