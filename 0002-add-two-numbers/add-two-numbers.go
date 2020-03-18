package solution

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func (l *ListNode) String() string {
	s := ""
	for l != nil {
		s = fmt.Sprintf("%s -> %d", s, l.Val)
		l = l.Next
	}
	if s == "" {
		return s
	}
	return s[4:]
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	rv := &ListNode{Val: 0, Next: nil}
	prev := rv
	var a, v int
	for a > 0 || l1 != nil || l2 != nil {
		v = a
		if l1 != nil {
			v += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			v += l2.Val
			l2 = l2.Next
		}
		a = v / 10
		v = v % 10
		curr := &ListNode{Val: v, Next: nil}
		prev.Next = curr
		prev = curr
	}
	return rv.Next
}
