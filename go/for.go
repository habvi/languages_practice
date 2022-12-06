package main

import "fmt"

func main() {
	for i := 0; i < 3; i++ {
		fmt.Println(i)
	}

	for i, v := range []int{12, 14, 16} {
		fmt.Println(i, v)
	}

	var i int
	for {
		fmt.Println(i, "imf loop")
		if i == 2 {
			break
		}
		i++
	}

LOOP:
	for {
		fmt.Println(i, "break rabel")
		switch {
		case i == 4:
			break LOOP
		}
		i++
	}
}
