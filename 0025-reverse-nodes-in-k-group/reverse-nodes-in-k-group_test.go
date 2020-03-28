package solution

import (
	"fmt"
	"testing"
)

func buildList(nums []int) *ListNode {
	head := &ListNode{}
	node := head
	for _, v := range nums {
		node.Next = &ListNode{v, nil}
		node = node.Next
	}
	return head.Next

}

func listToSlice(list *ListNode) []int {
	var nums []int
	for node := list; node != nil; node = node.Next {
		nums = append(nums, node.Val)
	}
	return nums
}
func TestReverseKGroup(t *testing.T) {
	tf := func(nums []int, k int, expected []int) {
		output := listToSlice(reverseKGroup(buildList(nums), k))
		if fmt.Sprintf("%v", output) != fmt.Sprintf("%v", expected) {
			t.Errorf("input: \"%v, %v\", expected: \"%v\", output: \"%v\"\n",
				nums, k, expected, output)
		}
	}

	tf([]int{1, 2, 3, 4, 5}, 2,
		[]int{2, 1, 4, 3, 5})
	tf([]int{1, 2, 3, 4, 5}, 3,
		[]int{3, 2, 1, 4, 5})
}
