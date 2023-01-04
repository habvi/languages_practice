package main

import (
	"container/list"
	"fmt"
)

func deque_print_all(deque *list.List) {
	for e := deque.Front(); e != nil; e = e.Next() {
		fmt.Print(e.Value, " ")
	}
	fmt.Println()
}

func deque_print_all_reverse(deque *list.List) {
	for e := deque.Back(); e != nil; e = e.Prev() {
		fmt.Print(e.Value, " ")
	}
	fmt.Println()
}

func PopFrontDeque(deque *list.List) interface{} {
	x := deque.Front()
	if x == nil {
		return nil
	}
	return deque.Remove(x)
}

func PopBackDeque(deque *list.List) interface{} {
	x := deque.Back()
	if x == nil {
		return nil
	}
	return deque.Remove(x)
}

func main() {
	deque := list.New()
	deque.PushBack(2)
	deque.PushBack(5)
	deque.PushBack(8)

	fmt.Println(deque.Len())

	deque_print_all(deque)
	deque_print_all_reverse(deque)

	fmt.Println(deque.Front().Value)
	fmt.Println(PopFrontDeque(deque))

	fmt.Println(deque.Back().Value)
	fmt.Println(PopBackDeque(deque))

}
