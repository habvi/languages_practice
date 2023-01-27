func PopFrontDeque(deque *list.List) interface{} {
	x := deque.Front()
	if x == nil {
		return nil
	}
	return deque.Remove(x)
}

func bfs(grid [][]byte, seen [][]bool, h, w, sy, sx int) [][]bool {
	q := list.New()
	q.PushBack([]int{sy, sx})
	seen := make([][]bool, h)
	for i := 0; i < h; i++ {
		seen[i] = make([]bool, w)
	}
	seen[sy][sx] = true
	for q.Len() > 0 {
		xy := PopFrontDeque(q)
		y, x := xy.([]int)[0], xy.([]int)[1]
		for _, dxy := range [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}} {
			ny, nx := dxy[0]+y, dxy[1]+x
			if !(0 <= ny && ny < h && 0 <= nx && nx < w) || grid[ny][nx] == '0' {
				continue
			}
			if seen[ny][nx] == true {
				continue
			}
			seen[ny][nx] = true
			q.PushBack([]int{ny, nx})
		}
	}
	return seen
}
