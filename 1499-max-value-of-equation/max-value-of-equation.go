package solution

func findMaxValueOfEquation(points [][]int, k int) int {
	// Monotonic Queue
	n := len(points)
	rv := -1 << 63
	monoque := make([][2]int, n)
	mqh := 0 // queue head
	mql := 0 // queue length
	for i := 0; i < n; i++ {
		x, y := points[i][0], points[i][1]
		// pop left while xi-xj > k
		for mql > 0 && monoque[mqh][1] < x-k {
			mqh++
			mql--
		}
		if mql > 0 {
			v := monoque[mqh][0] + y + x
			if v > rv {
				rv = v
			}
		}
		// pop right keep max
		for mql > 0 && monoque[mqh+mql-1][0] <= y-x {
			mql--
		}
		mql++
		monoque[mqh+mql-1][0] = y - x
		monoque[mqh+mql-1][1] = x
	}
	return rv
}

func findMaxValueOfEquationSimple(points [][]int, k int) int {
	length := len(points)
	maxV := -1 << 63
	for i := 0; i < length-1; i++ {
		for j := i + 1; j < length; j++ {
			x1, y1 := points[i][0], points[i][1]
			x2, y2 := points[j][0], points[j][1]
			if x2-x1 > k {
				continue
			}
			v := y1 + y2 + x2 - x1
			if v > maxV {
				maxV = v
			}
		}
	}
	return maxV
}
