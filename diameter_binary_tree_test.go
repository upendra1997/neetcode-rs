package main

import (
	"log/slog"
	"testing"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func diameterAndDepth(root *TreeNode) (int, int) {
	if root == nil {
		return -1, -1
	}
	diameter_l, depth_l := diameterAndDepth(root.Left)
	diameter_r, depth_r := diameterAndDepth(root.Right)
	depth := max(depth_l, depth_r) + 1
	diameter_through_depth := depth_l + depth_r + 2
	max_diameter := max(diameter_through_depth, diameter_l, diameter_r)
	slog.Info("tree", "value", root.Val, "depth", depth, "diameter", max_diameter)
	return max_diameter, depth
}

func diameterOfBinaryTree(root *TreeNode) int {
	diameter, _ := diameterAndDepth(root)
	return diameter
}

func Test(t *testing.T) {
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
	expected_result := 3
	actual_result := diameterOfBinaryTree(&tree)
	if expected_result != actual_result {
		t.Fatalf("FAILED expected: %v, actual: %v", expected_result, actual_result)
	}
}
