package solution

import "testing"

func TestFindMedianSortedArrays(t *testing.T) {
	tf := func(n1 []int, n2 []int, expected float64) {
		output := findMedianSortedArrays(n1, n2)

		t.Logf("expected: %v, output: %v", expected, output)
		if output != expected {
			t.Error()
		}
	}

	tf([]int{1, 2, 3}, []int{1, 2, 3}, 2.0)

	tf([]int{1, 3}, []int{2}, 2.0)
	tf([]int{1, 2}, []int{3, 4}, 2.5)

	tf([]int{1, 2}, []int{3}, 2.0)
	tf([]int{1, 3}, []int{2, 4}, 2.5)
}
