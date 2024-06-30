package main

import (
	// "log/slog"
	"testing"
)

type State int

const (
	NotVisited State = iota
	Processing
	Visited
)

func maxAreaOfIsland(grid [][]int) int {
	visited := make(map[[2]int]State, 0)
	areas := make(map[[2]int]int, 0)
	for x := range grid {
		for y := range grid {
			coord := [2]int{x, y}
			visited[coord] = NotVisited
			areas[coord] = 0
		}
	}

	var dfs func([2]int) int
	dfs = func(coord [2]int) int {
		res := visited[coord]
		switch res {
		case Visited:
			return areas[coord]
		case Processing:
			return 0
		}
		visited[coord] = Processing
		x := coord[0]
		y := coord[1]
		if grid[x][y] != 1 {
			visited[coord] = Visited
			areas[coord] = 0
			return 0
		}
		deltas := [][2]int{{0, -1}, {0, 1}, {1, 0}, {-1, 0}}
		area := 1
		for _, v := range deltas {
			dx := v[0]
			dy := v[1]
			new_x := x + dx
			new_y := y + dy
			if new_x < 0 || new_x >= len(grid) {
				continue
			}
			if new_y < 0 || new_y >= len(grid[0]) {
				continue
			}
			new_coord := [2]int{new_x, new_y}
			r := visited[new_coord]
			var a int
			switch r {
			case NotVisited:
				a = dfs(new_coord)
			}
			// slog.Info("dfs", "coord", coord, "new_coord", new_coord, "area", a)
			area += a
		}
		areas[coord] = area
		visited[coord] = Visited
		// slog.Info("dfs", "coord", coord, "area", area, "areas", areas, "visited", visited)
		return area
	}

	max_area := 0
	for x := range grid {
		for y := range grid[0] {
			coord := [2]int{x, y}
			r := visited[coord]
			switch r {
			case NotVisited:
				// slog.Info("main")
				max_area = max(max_area, dfs(coord))
			}
		}
	}
	return max_area
}

func TestMaxAreaOfIslands(t *testing.T) {
	input := [][]int{{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0}, {0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0}, {0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0}}
	result := maxAreaOfIsland(input)
	if result != 6 {
		t.Errorf("%v value not equal", result)
	}
}

func TestMaxAreaOfIslands_2(t *testing.T) {
	input := [][]int{{1, 1, 0, 0, 0}, {1, 1, 0, 0, 0}, {0, 0, 0, 1, 1}, {0, 0, 0, 1, 1}}
	result := maxAreaOfIsland(input)
	if result != 4 {
		t.Errorf("%v value not equal", result)
	}

}
