package main

import (
	"container/list"
	"fmt"
)

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
	fmt.Println(deque.Front().Value)
	fmt.Println(PopFrontDeque(deque))
	fmt.Println(deque.Back().Value)
	fmt.Println(PopBackDeque(deque))
}
