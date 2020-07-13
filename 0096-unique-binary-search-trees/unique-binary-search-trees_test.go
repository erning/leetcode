package solution

import (
	"testing"
)

func TestNumTrees(t *testing.T) {
	tf := func(input int, expected int) {
		output := numTrees(input)
		if output != expected {
			t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
				input, expected, output)
		}
	}

	tf(0, 0)
	tf(1, 1)
	tf(2, 2)
	tf(3, 5)
	tf(4, 14)
	tf(5, 42)
	// tf(8, 1430)
}
