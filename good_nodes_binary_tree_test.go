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
func goodNodes(root *TreeNode) int {
	type Node struct {
		node *TreeNode
		max  int
	}

	result := 0
	var queue []Node
	if root == nil {
		return result
	}
	queue = append(queue, Node{node: root, max: root.Val})
	for len(queue) != 0 {
		n := queue[0]
		m := n.max
		node := n.node
		if node.Val >= m {
			result += 1
		}
		queue = queue[1:]
		slog.Info("result", "node", n.node, "max", n.max, "queue", queue)
		if node.Left != nil {
			queue = append(queue, Node{node: node.Left, max: max(m, node.Left.Val)})
		}
		if node.Right != nil {
			queue = append(queue, Node{node: node.Right, max: max(m, node.Right.Val)})
		}
	}
	return result
}

func TestGoodNodes(t *testing.T) {
	node := TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val: 3,
			},
		},
		Right: &TreeNode{Val: 4,
			Left:  &TreeNode{Val: 1},
			Right: &TreeNode{Val: 5},
		},
	}
	result := goodNodes(&node)
	if result != 4 {
		t.Fatalf("expected: 4, actual: %v", result)
	}
}
