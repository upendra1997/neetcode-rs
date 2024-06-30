package main

import (
	"fmt"
	"strconv"
	"strings"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type Codec struct {
}

func constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	if root == nil {
		return "N"
	}
	return fmt.Sprintf("%d,%s,%s", root.Val, this.serialize(root.Left), this.serialize(root.Right))
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	var createNode func(node string) (*TreeNode, string)
	createNode = func(node string) (*TreeNode, string) {
		n, rest, ok := strings.Cut(node, ",")
		if !ok {
			return nil, ""
		}
		if n == "N" {
			return nil, rest
		}
		r, err := strconv.ParseInt(n, 10, 0)
		if err != nil {
			panic(err)
		}
		res := TreeNode{Val: int(r)}
		if len(rest) > 0 {
			r, c := createNode(rest)
			res.Left = r
			rest = c
		}
		if len(rest) > 0 {
			r, c := createNode(rest)
			res.Right = r
			rest = c
		}
		return &res, rest
	}
	result, rest := createNode(data)
	if len(rest) != 0 {
		panic("string was not consumed completely")
	}
	return result
}

/**
 * Your Codec object will be instantiated and called as such:
 * ser := Constructor();
 * deser := Constructor();
 * data := ser.serialize(root);
 * ans := deser.deserialize(data);
 */
