package main

import (
	// "log/slog"
	"slices"
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
func _buildTree(preorder []int, inorder []int) (*TreeNode, int) {
	if len(preorder) == 0 {
		return nil, 0
	}
	node := preorder[0]
	root := TreeNode{Val: node}
	index := slices.Index(inorder, node)
	left_inorder := inorder[:max(0, index)]
	right_inorder := inorder[min(len(inorder), index+1):]
	preorder = preorder[1:]
	if index == -1 {
		panic("anarth ho gaya")
	}
	count_l := 0
	count_r := 0
	if len(left_inorder) > 0 {
		root.Left, count_l = _buildTree(preorder, left_inorder)
		preorder = preorder[count_l:]
	}
	if len(right_inorder) > 0 {
		root.Right, count_r = _buildTree(preorder, right_inorder)
		preorder = preorder[count_r:]
	}
	return &root, count_l + count_r + 1
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	res, _ := _buildTree(preorder, inorder)
	return res
}
func TestBuildTree(t *testing.T) {
	buildTree([]int{3, 9, 20, 15, 7}, []int{9, 3, 15, 20, 7})
}
func TestBuildTree2(t *testing.T) {
	buildTree([]int{1, 2}, []int{1, 2})
}
func TestBuildTree3(t *testing.T) {
	buildTree([]int{3, 1, 2, 4}, []int{1, 2, 3, 4})
}
