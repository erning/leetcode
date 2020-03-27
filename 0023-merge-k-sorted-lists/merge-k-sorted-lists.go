package solution

import "sort"

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeKLists(lists []*ListNode) *ListNode {
	var p []int
	for _, list := range lists {
		for node := list; node != nil; node = node.Next {
			p = append(p, node.Val)
		}
	}
	sort.Slice(p, func(i, j int) bool {
		return p[i] < p[j]
	})
	head := &ListNode{}
	node := head
	for _, v := range p {
		node.Next = &ListNode{Val: v}
		node = node.Next
	}
	return head.Next
}
