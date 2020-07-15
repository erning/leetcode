package solution

import (
	"testing"
)

func TestCoinChange(t *testing.T) {
	tf := func(coins []int, amount int, expected int) {
		output := coinChange(coins, amount)
		if output != expected {
			t.Errorf("coins: \"%v\", amount: \"%v\", expected: \"%v\", output: \"%v\"\n",
				coins, amount, expected, output)
		}
	}

	tf([]int{1, 2, 5}, 11, 3)
	tf([]int{2}, 3, -1)
	tf([]int{186, 419, 83, 408}, 6249, 20)
}
