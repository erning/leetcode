package solution

import (
	"fmt"
	"testing"
)

func buildLists(input [][]int) []*ListNode {
	var lists []*ListNode
	for _, nums := range input {
		lists = append(lists, buildList(nums))
	}
	return lists
}

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
func TestMergeKLists(t *testing.T) {
	tf := func(input [][]int, expected []int) {
		output := listToSlice(mergeKLists(buildLists(input)))
		if fmt.Sprintf("%v", output) != fmt.Sprintf("%v", expected) {
			t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
				input, expected, output)
		}
	}

	tf([][]int{
		{1, 4, 5},
		{1, 3, 4},
		{2, 6},
	},
		[]int{1, 1, 2, 3, 4, 4, 5, 6},
	)
}
