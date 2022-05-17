package solution

// ListNode - Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, m int, n int) *ListNode {
	var prev, curr *ListNode
	curr = head
	for i := 1; curr != nil; i++ {
		if i < m {
			prev = curr
			curr = curr.Next
			continue
		}
		curr = reverseList(curr, n-m+1)
		if prev == nil {
			return curr
		}
		prev.Next = curr
		break
	}
	return head
}

func reverseList(head *ListNode, n int) *ListNode {
	var prev, curr *ListNode
	curr = head
	for i := 0; i < n && curr != nil; i++ {
		nextTemp := curr.Next
		curr.Next = prev
		prev = curr
		curr = nextTemp
	}
	head.Next = curr
	return prev
}
