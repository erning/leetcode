package solution

import (
	"fmt"
	"testing"
)

func TestTwoSum(t *testing.T) {
	num := []int{2, 7, 11, 15}
	target := 9
	expected := []int{0, 1}
	output := twoSum(num, target)

	t.Logf("expected: %v, output: %v", expected, output)
	if fmt.Sprintf("%v", output) != fmt.Sprintf("%v", expected) {
		t.Error()
	}
}
