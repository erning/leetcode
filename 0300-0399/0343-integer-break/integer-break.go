package solution

func integerBreak(n int) int {
	dp := make([]int, n+1)
	for i := 2; i <= n; i++ {
		dp[i] = i - 1
		for j := 1; j <= i/2; j++ {
			a, b := j, i-j
			if a < dp[a] {
				a = dp[a]
			}
			if b < dp[b] {
				b = dp[b]
			}
			c := a * b
			if c > dp[i] {
				dp[i] = c
			}
		}
	}
	return dp[n]
}
