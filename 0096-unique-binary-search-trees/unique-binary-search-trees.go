package solution

func numTrees(n int) int {
	return numTreesAB(n)
}

func numTreesAB(n int) int {
	if n == 0 {
		return 0
	}

	cache := make([]int, n)
	var numTreeAB func(int, int) int
	numTreeAB = func(a, b int) int {
		if a > b {
			return 1
		}
		if cache[b-a] > 0 {
			return cache[b-a]
		}

		ans := 0
		for m := a; m <= b; m++ {
			x := numTreeAB(a, m-1)
			y := numTreeAB(m+1, b)
			ans += x * y
		}
		cache[b-a] = ans
		return ans
	}
	return numTreeAB(1, n)
}
