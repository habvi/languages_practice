package main

import (
	"container/list"
	"fmt"
)

type Stack struct {
	v *list.List
}

func NewStack() *Stack {
	return &Stack{v: list.New()}
}

func (s *Stack) Push(v interface{}) {
	s.v.PushBack(v)
}

func (s *Stack) Pop() interface{} {
	b := s.v.Back()
	if b == nil {
		return nil
	}
	return s.v.Remove(b)
}

func main() {
	stack := NewStack()
	stack.Push(2)
	stack.Push(5)
	stack.Push(8)
	fmt.Println(stack.Pop())
	fmt.Println(stack.Pop())
}
