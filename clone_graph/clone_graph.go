package main

import (
	"log/slog"
)

type State int

const (
	NotVisited State = iota
	Processing
	Visited
)

type Node struct {
	Val       int
	Neighbors []*Node
}

var visited map[*Node]State
var cache map[*Node]*Node

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */
func cloneGraph_(node *Node) *Node {
	if node == nil {
		return nil
	}
	res, ok := visited[node]
	if !ok {
		visited[node] = NotVisited
		res = NotVisited
	}
	switch res {
	case Visited | Processing:
		return cache[node]
	}

	visited[node] = Processing

	new_node := Node{
		Val:       node.Val,
		Neighbors: make([]*Node, 0),
	}

	cache[node] = &new_node

	for _, v := range node.Neighbors {
		res, ok := visited[v]
		if !ok {
			res = NotVisited
		}
		var ref *Node
		switch res {
		case Processing, Visited:
			ref = cache[v]
		case NotVisited:
			ref = cloneGraph_(v)
		default:
			panic("wtf")
		}
		cache[node].Neighbors = append(cache[node].Neighbors, ref)
	}
	visited[node] = Visited
	return cache[node]
}

func cloneGraph(node *Node) *Node {
	visited = make(map[*Node]State)
	cache = make(map[*Node]*Node)
	result := cloneGraph_(node)
	for k, v := range cache {
		slog.Info("old", "node", k, "value", k.Val, "neighbor", k.Neighbors)
		slog.Info("new", "node", v, "value", v.Val, "neighbor", v.Neighbors)
	}
	slog.Info("cloneGraph", "visited", visited)
	slog.Info("cloneGraph", "cache", cache)
	return result
}
