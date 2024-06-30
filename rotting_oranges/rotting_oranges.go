package main

// import "log/slog"

type Orange int

const (
	NoOrange Orange = iota
	FreshOrange
	RottenOrange
)

type State int

const (
	NotVisited State = iota
	Processing
	Visited
)

type Coord = [2]int

func emptyGrid(grid [][]int) bool {
	for x := range grid {
		for y := range grid[x] {
			switch grid[x][y] {
			case int(RottenOrange), int(FreshOrange):
				return false
			}
		}
	}
	return true
}

func orangesRotting(grid [][]int) int {
	visited := make(map[Coord]State)
	cache := make(map[Coord]int)
	queue := make([]Coord, 0)
	parent := make(map[Coord]*Coord)
	for x := range grid {
		for y := range grid[x] {
			visited[[2]int{x, y}] = NotVisited
			switch grid[x][y] {
			case int(RottenOrange):
				coord := [2]int{x, y}
				cache[coord] = 0
				parent[coord] = nil
				queue = append(queue, coord)
			}
		}
	}

	deltas := []Coord{{0, -1}, {0, 1}, {1, 0}, {-1, 0}}
	if emptyGrid(grid) {
		return 0
	}
	for len(queue) > 0 {
    // slog.Info("bfs", "queue", queue)
		coord := queue[0]
		queue = queue[1:]
		visited[coord] = Visited
		if parent[coord] != nil {
			cache[coord] = cache[*parent[coord]] + 1
		}
		x := coord[0]
		y := coord[1]
		grid[x][y] = int(RottenOrange)
		for _, d := range deltas {
			dx := d[0]
			dy := d[1]
			new_x := x + dx
			new_y := y + dy
			if new_x < 0 || new_x >= len(grid) {
				continue
			}
			if new_y < 0 || new_y >= len(grid[0]) {
				continue
			}
			new_coord := Coord{new_x, new_y}
			switch visited[new_coord] {
			case NotVisited:
				switch grid[new_x][new_y] {
				case int(FreshOrange):
          visited[new_coord] = Visited
          // slog.Info("magic", "parent", coord, "child", new_coord)
          parent[new_coord] = &coord
					queue = append(queue, new_coord)
				}
			}
		}
	}

	m := -1
	for _, v := range cache {
		m = max(v, m)
	}

	for x := range grid {
		for y := range grid[x] {
			switch grid[x][y] {
			case int(FreshOrange):
				return -1
			}
		}
	}

  // slog.Info("BFS", "cache", cache)
  // slog.Info("BFS", "parent", parent)

	return m
}
