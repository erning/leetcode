package solution

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	a, b := head, head.Next
	a.Next, b.Next = swapPairs(b.Next), a
	return b
}
