package solution

func findMaxValueOfEquation(points [][]int, k int) int {
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
