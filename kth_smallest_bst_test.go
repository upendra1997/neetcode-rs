package main

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func kthSmallest(root *TreeNode, k int) int {
	var inOrder func(*TreeNode, int) int
	var result *TreeNode
	inOrder = func(root *TreeNode, element_processed int) int {
		if result != nil {
			return element_processed
		}
		if root == nil {
			return element_processed
		}
		res := inOrder(root.Left, element_processed)
		if res == k-1 {
			result = root
		}
		res = inOrder(root.Right, res+1)
		return res
	}
	_ = inOrder(root, 0)
	return result.Val
}
