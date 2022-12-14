package main

import (
	"container/list"
	"fmt"
)

func PopStack(stack *list.List) interface{} {
	x := stack.Back()
	if x == nil {
		return nil
	}
	return stack.Remove(x)
}

func main() {
	stack := list.New()
	stack.PushBack(2)
	stack.PushBack(5)
	stack.PushBack(8)
	fmt.Println(stack.Back().Value)
	fmt.Println(PopStack(stack))
	fmt.Println(PopStack(stack))
}
