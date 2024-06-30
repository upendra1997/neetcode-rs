package main

import "testing"

func TestHello(t *testing.T) {
	grid := [][]int{{2, 1, 1}, {1, 1, 0}, {0, 1, 1}}
	result := orangesRotting(grid)
	if result != 4 {
		t.Errorf("%v not equal to 4", result)
	}
}
func TestEmpty(t *testing.T) {
	grid := [][]int{{0}}
	result := orangesRotting(grid)
	if result != 0 {
		t.Errorf("%v not equal to 0", result)
	}
}

func TestFreshOrange(t *testing.T) {
	grid := [][]int{{1}}
	result := orangesRotting(grid)
	if result != -1 {
		t.Errorf("%v not equal to -1", result)
	}
}

func TestMultipleParent(t *testing.T) {
	grid := [][]int{{2, 2}, {1, 1}, {0, 0}, {2, 0}}
	result := orangesRotting(grid)
	if result != 1 {
		t.Errorf("%v not equal to 1", result)
	}
}
