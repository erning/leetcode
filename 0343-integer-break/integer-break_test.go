package solution

import (
	"testing"
)

func TestIntegerBreak(t *testing.T) {
	tf := func(n int, expected int) {
		output := integerBreak(n)
		if output != expected {
			t.Errorf("n: \"%v\", expected: \"%v\", output: \"%v\"\n",
				n, expected, output)
		}
	}

	tf(2, 1)
	tf(3, 2)
	tf(4, 4)
	tf(6, 9)
	tf(10, 36)
	tf(8, 18)
}
