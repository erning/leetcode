package solution

const max = 1<<31 - 1

func coinChange(coins []int, amount int) int {
	dp := make([]int, amount+1)
	for i := 1; i < len(dp); i++ {
		dp[i] = max
	}
	for i := 0; i < len(dp); i++ {
		for _, coin := range coins {
			v := i - coin
			if v < 0 {
				continue
			}
			if dp[i] > dp[v]+1 {
				dp[i] = dp[v] + 1
			}
		}
	}
	if dp[amount] >= max {
		return -1
	}
	return dp[amount]
}

func coinChangeRecursion(coins []int, amount int) int {
	memo := make(map[int]int)
	var dp func(int) int
	dp = func(amount int) int {
		if v, ok := memo[amount]; ok {
			return v
		}
		if amount == 0 {
			return 0
		}
		if amount < 0 {
			return max
		}
		min := max
		for _, coin := range coins {
			v := dp(amount - coin)
			if v >= max {
				continue
			}
			if v < min {
				min = 1 + v
			}
		}
		memo[amount] = min
		return min
	}
	v := dp(amount)
	if v >= max {
		return -1
	}
	return v
}
