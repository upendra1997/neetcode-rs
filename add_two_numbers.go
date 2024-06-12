package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var newNode *ListNode
	carry := 0
	temp := newNode
	for l1 != nil && l2 != nil {
		result := l1.Val + l2.Val + carry
		carry = result / 10
		if newNode == nil {
			newNode = &ListNode{Val: result % 10}
			temp = newNode
		} else {
			temp.Next = &ListNode{Val: result % 10}
			temp = temp.Next
		}
		l1 = l1.Next
		l2 = l2.Next
	}

	for l1 != nil {
		result := l1.Val + carry
		carry = result / 10
		if newNode == nil {
			newNode = &ListNode{Val: result % 10}
			temp = newNode
		} else {
			temp.Next = &ListNode{Val: result % 10}
			temp = temp.Next
		}
		l1 = l1.Next
	}

	for l2 != nil {
		result := l2.Val + carry
		carry = result / 10
		if newNode == nil {
			newNode = &ListNode{Val: result % 10}
			temp = newNode
		} else {
			temp.Next = &ListNode{Val: result % 10}
			temp = temp.Next
		}
		l2 = l2.Next
	}

	if carry != 0 {
		if newNode == nil {
			newNode = &ListNode{Val: carry}
			temp = newNode
		} else {
			temp.Next = &ListNode{Val: carry}
			temp = temp.Next
		}
	}

	return newNode
}
