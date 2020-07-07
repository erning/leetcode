package solution

func minInteger(num string, k int) string {
	numBytes := []byte(num)
	length := len(numBytes)
	var b int

	_min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	_findMinDigit := func() int {
		if numBytes[b] == '0' {
			return b
		}
		e := _min(length-1, b+k)
		minV := numBytes[b]
		minI := b
		for i := b + 1; i <= e; i++ {
			if numBytes[i] < minV {
				minV = numBytes[i]
				minI = i
			}
			if minV == '0' {
				break
			}
		}
		return minI
	}

	for b = 0; b < length && k > 0; b++ {
		i := _findMinDigit()
		if i == b {
			continue
		}
		v := numBytes[i]
		copy(numBytes[b+1:], numBytes[b:i])
		numBytes[b] = v
		k -= i - b
	}
	return string(numBytes)
}
