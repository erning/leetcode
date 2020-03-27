package solution

func maxSizeSlices(slices []int) int {
	slen := len(slices)
	C := slen / 3
	// W := 1

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	m0 := make([][]int, slen-1) // pick first number
	m1 := make([][]int, slen-1) // do not pick the first number

	for i := 0; i < 2 && i < slen-1; i++ {
		m0[i] = make([]int, C+1)
		m1[i] = make([]int, C+1)
		for w := 1; w < C+1; w++ {
			m0[i][w] = slices[i]
			m1[i][w] = slices[i+1]
		}
	}

	for i := 2; i < slen-1; i++ {
		m0[i] = make([]int, C+1)
		m1[i] = make([]int, C+1)
		for w := 1; w < C+1; w++ {
			if i >= 3 {
				m0[i-2][w] = max(m0[i-3][w], m0[i-2][w])
				m1[i-2][w] = max(m1[i-3][w], m1[i-2][w])
			}
			m0[i][w] = max(m0[i-2][w], slices[i]+m0[i-2][w-1])
			m1[i][w] = max(m1[i-2][w], slices[i+1]+m1[i-2][w-1])
		}
	}

	// for i := slen - 2; i >= 0; i-- {
	// 	fmt.Printf("%v - %v: %v\n", i, slices[i], m0[i])
	// }
	// for i := slen - 2; i >= 0; i-- {
	// 	fmt.Printf("%v - %v: %v\n", i+1, slices[i+1], m1[i])
	// }
	// fmt.Printf("\n")

	a1, a2, b1, b2 := m0[slen-3][C], m0[slen-2][C], m1[slen-3][C], m1[slen-2][C]
	return max(max(max(a1, a2), b1), b2)
}
