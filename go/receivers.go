package main

import (
	"fmt"
)

// ------------------------------
type Hex int

func (h Hex) String() string {
	return fmt.Sprintf("%x", int(h))
}

func func1() {
	var hex Hex = 100
	fmt.Println(hex, hex.String())

	// var f func() string
	f := hex.String
	fmt.Println(f()) // 100(64)
	hex = 200
	fmt.Println(f()) // 100(64)

	var hp *Hex = &hex
	f = hp.String
	*hp = 200
	fmt.Println(f()) // 200(c8)

	// var f2 func(Hex) string
	f2 := Hex.String
	fmt.Printf("%T %s\n", f2, f2(hex))
}

// ------------------------------
type T int

func (t *T) f() {
	println("hello")
}

func func2() {
	var v T
	// same
	(&v).f()
	v.f() // only val
}

// ------------------------------

// func (t T) f1()  {}

// func (t *T) g1() {}

// func func3() {
// 	(T{}).f1()   // T
// 	(&T{}).f1()  // *T
// 	(*&T{}).f1() // T

// 	(T{}).g1() // NG
// 	(&T{}).g1()
// 	(*&T{}.g1())
// }

// ------------------------------
type Myint int

func (n *Myint) Inc() { *n++ }

func func4() {
	var n Myint
	println(n)
	n.Inc()
	println(n)
}

// ------------------------------

// ------------------------------
func main() {
	func1()
	func2()
	// func3()
	func4()
}
