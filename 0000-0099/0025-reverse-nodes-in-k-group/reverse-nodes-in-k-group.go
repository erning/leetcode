package solution

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

// reverse the list and returns the tail
// [prev -> 1 -> 2 -> 3 -> 4 -> 5]
// [prev -> 5 -> 4 -> 3 -> 2 -> 1]
func reverse(prev *ListNode, n int) (bool, *ListNode) {
	if prev == nil {
		return false, nil
	}
	if n <= 1 {
		return true, prev.Next
	}
	a := prev.Next
	if a == nil {
		return false, nil
	}
	b := a.Next
	if b == nil {
		return false, a
	}
	c := b.Next
	a.Next = nil
	var i int
	for i = 1; i < n; i++ {
		b.Next = a
		// fmt.Printf("%v: %v %v\n", i, a, b)
		a = b
		b = c
		if c == nil {
			break
		}
		c = c.Next
	}
	tail := prev.Next
	tail.Next = b
	prev.Next = a
	return i == n, tail
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	head = &ListNode{
		Next: head,
	}

	h := head
	for {
		ok, tail := reverse(h, k)
		if !ok {
			reverse(h, k)
			break
		}
		h = tail
	}

	// for h := head.Next; h != nil; h = h.Next {
	// 	fmt.Printf("%v\n", h)
	// }

	return head.Next
}
