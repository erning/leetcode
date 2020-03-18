package solution

import (
	"testing"
)

func buildListNode(num int) *ListNode {
	rv := &ListNode{Val: 0, Next: nil}
	prev := rv
	for num > 0 {
		curr := &ListNode{Val: num % 10, Next: nil}
		prev.Next = curr
		prev = curr
		num /= 10
	}
	return rv.Next
}

func TestTwoSum(t *testing.T) {
	tf := func(n1, n2 int) {
		l1 := buildListNode(n1)
		l2 := buildListNode(n2)
		expected := buildListNode(n1 + n2)
		output := addTwoNumbers(l1, l2)
		if output.String() != expected.String() {
			t.Errorf("expect: %v, output: %v", expected, output)
		}
		t.Logf("expect: %v, output: %v", expected, output)
	}

	tf(342, 456)

	tf(5, 5)
	tf(50, 6)
	tf(1001, 3002)
}

func BenchmarkTwoSum(t *testing.B) {
	l1 := buildListNode(342)
	l2 := buildListNode(465)

	for i := 0; i < 10000; i++ {
		addTwoNumbers(l1, l2)
	}
}
