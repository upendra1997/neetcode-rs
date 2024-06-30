package main

import "log/slog"

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func (head *ListNode) printList() {
	for temp := head; temp != nil; {
		slog.Info("List", "Node", temp.Val)
		temp = temp.Next
	}
}

func getKthNode(head *ListNode, k int) *ListNode {
	var prev *ListNode
	for temp := head; temp != nil && k != 0; {
		prev = temp
		temp = temp.Next
		k -= 1
	}
	if k == 0 {
		return prev
	}
	return nil
}

func reverse(head *ListNode, tail *ListNode) (*ListNode, *ListNode) {
	orginalHead := head
	prev := head
	next := head.Next
	for next != tail {
		temp := next.Next
		next.Next = prev
		prev = next
		next = temp
	}
	orginalHead.Next = nil
	return prev, orginalHead
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	HEAD := &ListNode{Val: 0, Next: head}
	dummy := HEAD
	iter := 0
	for {
		kthNode := getKthNode(head, k)
		if kthNode == nil {
			break
		}
		var tail *ListNode
		newHead := kthNode.Next
		head, tail = reverse(head, newHead)
		// slog.Info("node state:", "head", head, "tail", tail, "kthNode", kthNode)
		dummy.Next = head
		dummy = tail
		tail.Next = newHead
		head = newHead
		iter += 1
	}
	return HEAD.Next
}

// func main() {
// 	list := &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: &ListNode{Val: 5, Next: nil}}}}}
// 	// list.reverse(nil).printList()
// 	// list.printList()
// 	reverseKGroup(list, 2).printList()
// }
