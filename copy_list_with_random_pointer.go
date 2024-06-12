package main

// Definition for a Node.
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
  mapping := make(map[*Node]*Node)
	for temp := head; temp != nil; {
    mapping[temp] = &Node{Val: temp.Val}
		temp = temp.Next
	}
  mapping[nil] = nil
  
	for temp := head; temp != nil; {
    mapping[temp].Next = mapping[temp.Next]
    mapping[temp].Random = mapping[temp.Random]
		temp = temp.Next
	}

	return mapping[head]
}
