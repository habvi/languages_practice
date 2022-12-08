package main

import "fmt"

func f1(x, y int) (int, int) {
	return x + y, x - y
}

func f2() {
	msg := "abcd"
	func() {
		println(msg)
	}()
}

func f3() {
	fs := make([]func() string, 2)
	fs[0] = func() string { return "abc" }
	fs[1] = func() string { return "def" }
	for _, f := range fs {
		fmt.Println(f())
	}
}

func f4() {
	for i := 0; i < 3; i++ {
		func(i int) { fmt.Println(i) }(i)
	}
}

func main() {
	fmt.Println(f1(8, 5))
	f2()
	f3()
	f4()
}
