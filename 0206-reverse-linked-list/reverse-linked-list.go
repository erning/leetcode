package solution

// ListNode - Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var prev, curr *ListNode
	curr = head
	for curr != nil {
		nextTemp := curr.Next
		curr.Next = prev
		prev = curr
		curr = nextTemp
	}
	return prev
}
