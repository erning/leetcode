package solution

import (
	"reflect"
	"testing"
)

func sliceToList(input []int) *ListNode {
	head := &ListNode{}
	prev := head
	for i := 0; i < len(input); i++ {
		prev.Next = &ListNode{input[i], nil}
		prev = prev.Next
	}
	return head.Next
}

func listToSlice(input *ListNode) []int {
	var rv []int
	for curr := input; curr != nil; curr = curr.Next {
		rv = append(rv, curr.Val)
	}
	return rv
}

func TestReverseList(t *testing.T) {
	tf := func(input []int, expected []int) {
		inputList := sliceToList(input)
		outputList := reverseList(inputList)
		output := listToSlice(outputList)
		if !reflect.DeepEqual(output, expected) {
			t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
				input, expected, output)
		}
	}

	tf([]int{1, 2, 3, 4, 5}, []int{5, 4, 3, 2, 1})
}
