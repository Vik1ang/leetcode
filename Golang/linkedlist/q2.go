package linkedlist

import (
	"go_leetcode/data_structure"
)

type ListNode = data_structure.ListNode

type Solution1 struct{}

func (s *Solution1) addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var tail *ListNode
	var head *ListNode
	carry := 0

	for l1 != nil || l2 != nil {
		node1, node2 := 0, 0
		if l1 != nil {
			node1 = l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			node2 = l2.Val
			l2 = l2.Next
		}
		sum := node1 + node2 + carry
		carry = sum / 10

		if head == nil {
			head = &ListNode{Val: sum}
			tail = head
		} else {
			tail.Next = &ListNode{Val: sum}
			tail = tail.Next
		}
	}

	if carry > 0 {
		tail.Next = &ListNode{Val: carry}
	}

	return head
}
