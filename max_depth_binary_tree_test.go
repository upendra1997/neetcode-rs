package main

import (
	// "math"
	// "testing"
)

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	left_depth := maxDepth(root.Left)
	right_depth := maxDepth(root.Right)
	return 1 + max(left_depth, right_depth)
}

