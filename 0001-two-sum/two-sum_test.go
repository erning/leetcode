package solution

import (
	"fmt"
	"testing"
)

func TestTwoSum(t *testing.T) {
	num := []int{2, 7, 11, 15}
	target := 9

	result := twoSum(num, target)
	fmt.Printf("%v\n", result)
}
