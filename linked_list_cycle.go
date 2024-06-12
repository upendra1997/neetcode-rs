package main

// Definition for singly-linked list.
// type ListNode struct {
// 	Val  int
// 	Next *ListNode
// }

func hasCycle(head *ListNode) bool {
	slow := head
	fast := head

	if fast != nil {
		fast = fast.Next
	}

	for slow != fast {
		if slow != nil {
			slow = slow.Next
		}
		if fast != nil {
			fast = fast.Next
		}
		if fast != nil {
			fast = fast.Next
		}
	}
	return slow != nil
}
