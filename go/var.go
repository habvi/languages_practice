package main

import "fmt"

func main() {
	var x int
	var y uint16 = 10
	z := 20
	fmt.Printf("%v, type: %T\n", x, x)
	fmt.Printf("%v, type: %T\n", y, y)
	fmt.Printf("%v, type: %T\n", z, z)

	const a float32 = 4.32
	const (
		b        = 5.0
		c uint32 = 8
		d        = "abc"
		e bool   = true
	)
	fmt.Printf("%v, type: %T\n", a, a)
	fmt.Printf("%v, type: %T\n", b, b)
	fmt.Printf("%v, type: %T\n", c, c)
	fmt.Printf("%v, type: %T\n", d, d)
	fmt.Printf("%v, type: %T\n", e, e)
}
