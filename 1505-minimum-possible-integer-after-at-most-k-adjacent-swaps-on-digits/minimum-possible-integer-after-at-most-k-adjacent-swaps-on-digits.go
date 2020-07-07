package solution

func minInteger(num string, k int) string {
	numBytes := []byte(num)
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
		e := _min(len(numBytes)-1, b+k)
		maxV := numBytes[b]
		maxI := b
		for i := b + 1; i <= e; i++ {
			if numBytes[i] < maxV {
				maxV = numBytes[i]
				maxI = i
			}
			if maxV == '0' {
				break
			}
		}
		return maxI
	}

	for b = 0; b < len(num) && k > 0; b++ {
		i := _findMinDigit()
		if i == b {
			continue
		}
		for j := i; j > b; j-- {
			numBytes[j], numBytes[j-1] = numBytes[j-1], numBytes[j]
		}
		k -= i - b
	}
	return string(numBytes)
}
