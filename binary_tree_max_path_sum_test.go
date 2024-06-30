package main

import (
	"log/slog"
	"math"
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

func _maxPathSum(root *TreeNode) (sumWithoutSplitting, sumWithSplitting int64) {
	var minInt int32
	minInt = math.MinInt32
	if root == nil {
		return int64(minInt), 0
	}
	wol, wl := _maxPathSum(root.Left)
	slog.Info("test max path sum", "node", root.Val, "wol", wol, "wl", wl)
	wor, wr := _maxPathSum(root.Right)
	slog.Info("test max path sum", "node", root.Val, "wor", wor, "wr", wr)
	wl = max(0, wl)
	wr = max(0, wr)
	w := wl + wr + int64(root.Val)
	return max(0, wl, wr) + int64(root.Val), w
}

func maxPathSum(root *TreeNode) int {
	_, res := _maxPathSum(root)
	return int(res)
}

func TestMaxPathSum(t *testing.T) {
	slog.Info("test max path sum", "result", maxPathSum(&TreeNode{Val: -2, Left: &TreeNode{Val: 1}}))
}
