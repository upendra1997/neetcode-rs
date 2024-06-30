package main

import (
	"log/slog"
	"testing"
)

func isValidBST(root *TreeNode) bool {
	result := []int{}
	var inOrder func(root *TreeNode)
	inOrder = func(root *TreeNode) {
		if root == nil {
			return
		}
		inOrder(root.Left)
		result = append(result, root.Val)
		inOrder(root.Right)
	}
	inOrder(root)
	slog.Info("isValidBST", "inorder", result)
	for i, j := 0, 2; j <= len(result); i, j = i+1, j+1 {
		if result[i:j][1] <= result[i:j][0] {
			return false
		}
	}
	return true
}

func TestIsValidBst(t *testing.T) {
	root := &TreeNode{
		Val:   2,
		Left:  &TreeNode{Val: 1},
		Right: &TreeNode{Val: 3},
	}
	res := isValidBST(root)
	if res != true {
		t.Fatalf("got return result")
	}
}
