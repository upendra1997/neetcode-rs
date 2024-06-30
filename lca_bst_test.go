package main

import "log/slog"

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val   int
 *     Left  *TreeNode
 *     Right *TreeNode
 * }
 */

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil || p == nil || q == nil {
		return root
	}
	if root == p || root == q {
		return root
	}
	var a, b *TreeNode
	if p.Val < q.Val {
		a, b = p, q
	} else {
		a, b = q, p
	}
	slog.Info("printing node information", "root", root.Val, "left", root.Left, "right", root.Right)
	if a.Val < root.Val && b.Val > root.Val {
		return root
	} else if a.Val < root.Val && b.Val < root.Val {
		return lowestCommonAncestor(root.Left, a, b)
	} else if a.Val > root.Val && b.Val > root.Val {
		return lowestCommonAncestor(root.Right, a, b)
	}
	panic("unreachable")
}
