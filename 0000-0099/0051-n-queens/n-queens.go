package solution

func solveNQueens(n int) [][]string {
	var (
		m1 []bool = make([]bool, n)   // column
		m2 []bool = make([]bool, n)   // rows
		m3 []bool = make([]bool, n*2) // slash
		m4 []bool = make([]bool, n*2) // backslash
	)
	var board [][]bool = make([][]bool, n)
	for i := 0; i < n; i++ {
		board[i] = make([]bool, n)
	}

	var ret [][]string

	var solve func(int)
	solve = func(y int) {
		if y >= n {
			rv := make([]string, n)
			for i := 0; i < n; i++ {
				for j := 0; j < n; j++ {
					if board[j][i] {
						rv[i] += "Q"
					} else {
						rv[i] += "."
					}
				}
			}
			ret = append(ret, rv)
			return
		}
		for x := 0; x < n; x++ {
			if m1[x] || m2[y] || m3[x+y] || m4[n+x-y] {
				continue
			}
			board[x][y] = true
			m1[x], m2[y], m3[x+y], m4[n+x-y] = true, true, true, true
			solve(y + 1)
			board[x][y] = false
			m1[x], m2[y], m3[x+y], m4[n+x-y] = false, false, false, false
		}
	}

	solve(0)

	return ret
}
