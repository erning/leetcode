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
	n := len(lists)
	if n <= 0 {
		return nil
	}
	if n <= 1 {
		return lists[0]
	}
	for interval := 1; interval < n; interval = interval << 1 {
		for i := 0; i < n-interval; i = i + interval<<1 {
			lists[i] = merge2List(lists[i], lists[i+interval])
		}
	}
	return lists[0]
}

func merge2List(list1, list2 *ListNode) *ListNode {
	head := &ListNode{}
	node := head
	for ; list1 != nil && list2 != nil; node = node.Next {
		if list1.Val < list2.Val {
			node.Next = list1
			list1 = list1.Next
		} else {
			node.Next = list2
			list2 = list2.Next
		}
	}
	if list1 == nil {
		node.Next = list2
	} else {
		node.Next = list1
	}
	return head.Next
}

func mergeKListsTrick(lists []*ListNode) *ListNode {
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
