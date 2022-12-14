package main

import (
	"container/list"
	"fmt"
)

type Deque struct {
	v *list.List
}

func NewDeque() *Deque {
	return &Deque{v: list.New()}
}

func (d *Deque) Push(v interface{}) {
	d.v.PushBack(v)
}

func (d *Deque) Pop() interface{} {
	b := d.v.Front()
	if b == nil {
		return nil
	}
	return d.v.Remove(b)
}

func main() {
	deque := NewDeque()
	deque.Push(2)
	deque.Push(5)
	deque.Push(8)
	fmt.Println(deque.Pop())
	fmt.Println(deque.Pop())
}
