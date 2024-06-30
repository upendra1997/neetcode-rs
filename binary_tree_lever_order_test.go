package main

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func levelOrder(root *TreeNode) [][]int {
	type Node struct {
		node *TreeNode
		dis  int
	}

	var result [][]int
	var queue []Node
	if root == nil {
		return result
	}
	queue = append(queue, Node{node: root, dis: 0})
	for len(queue) != 0 {
		n := queue[0]
		d := n.dis
		node := n.node
		queue = queue[1:]
		if node.Left != nil {
			queue = append(queue, Node{node: node.Left, dis: d + 1})
		}
		if node.Right != nil {
			queue = append(queue, Node{node: node.Right, dis: d + 1})
		}
		for len(result) <= d {
			result = append(result, []int{})
		}
		result[d] = append(result[d], node.Val)
	}
	return result
}
