package main

import (
	"math"
	"testing"
)

func depthAndBalanced(root *TreeNode) (int, bool) {
	if root == nil {
		return -1, true
	}
	left, balanced_l := depthAndBalanced(root.Left)
	if !balanced_l {
		return left, false
	}
	right, balanced_r := depthAndBalanced(root.Right)
	depth := max(left, right) + 1
	if !balanced_r {
		return depth, false
	}
	if math.Abs(float64(right-left)) > 1 {
		return depth, false
	}
	return depth, true
}
func isBalanced(root *TreeNode) bool {
	_, b := depthAndBalanced(root)
	return b
}

func TestBalanced(t *testing.T) {
	tree := TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: &TreeNode{Val: 5},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}
	expected_result := false
	actual_result := isBalanced(&tree)
	if expected_result != actual_result {
		t.Fatalf("FAILED expected: %v, actual: %v", expected_result, actual_result)
	}
}
