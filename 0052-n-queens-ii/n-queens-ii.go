package solution

func totalNQueens(n int) int {
	var (
		m1 []bool = make([]bool, n)   // column
		m2 []bool = make([]bool, n)   // rows
		m3 []bool = make([]bool, n*2) // slash
		m4 []bool = make([]bool, n*2) // backslash
	)
	var ret int
	var solve func(int)
	solve = func(y int) {
		if y >= n {
			ret++
			return
		}
		for x := 0; x < n; x++ {
			if m1[x] || m2[y] || m3[x+y] || m4[n+x-y] {
				continue
			}
			m1[x], m2[y], m3[x+y], m4[n+x-y] = true, true, true, true
			solve(y + 1)
			m1[x], m2[y], m3[x+y], m4[n+x-y] = false, false, false, false
		}
	}
	solve(0)
	return ret
}
