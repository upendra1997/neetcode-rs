package main

import "math"

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func pop_first(this **ListNode) int {
	if this == nil {
		panic("trying to pop from empty list")
	}
	res := (*this).Val
	*this = (*this).Next
	return res
}

func insert(this **ListNode, val int) {
	if *this == nil {
		*this = &ListNode{Val: val, Next: nil}
	} else {
		(*this).Next = &ListNode{Val: val, Next: nil}
	}
}

func mergeKLists(lists []*ListNode) *ListNode {
	var result *ListNode
	var head *ListNode
	for {
		min_index := 0
		min_value := math.MaxInt
		for k, v := range lists {
			if v == nil {
				continue
			}
			if v.Val < min_value {
				min_value = v.Val
				min_index = k
			}
		}

		if min_value == math.MaxInt {
			break
		}
		insert(&result, pop_first(&lists[min_index]))
		if result != nil && head == nil {
			head = result
		} else {
			result = result.Next
		}
	}
	return head
}
