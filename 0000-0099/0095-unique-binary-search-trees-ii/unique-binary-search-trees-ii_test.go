package solution

import (
	"fmt"
	"testing"
)

func TestGenerateTrees(t *testing.T) {
	// tf := func(input int, expected []int) {
	// output := generateTrees(input)
	// if output != expected {
	// 	t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
	// 		input, expected, output)
	// }
	// }

	var ans []*TreeNode
	ans = generateTrees(4)
	for _, node := range ans {
		fmt.Printf("%v\n", node)
	}
}
