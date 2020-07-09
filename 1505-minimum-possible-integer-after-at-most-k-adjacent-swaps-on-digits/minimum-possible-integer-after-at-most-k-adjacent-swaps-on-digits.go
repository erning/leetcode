package solution

import (
	"math"
)

func minInteger(num string, k int) string {
	n := len(num)

	digits := [10][]int{}
	for i := 0; i < n; i++ {
		v := num[i] - '0'
		digits[v] = append(digits[v], i)
	}
	counter := MakeCounter(n)

	used := make([]bool, n)
	rv := make([]byte, n)
	p := 0
	for k > 0 {
		i, c := func() (int, int) {
			for v := 0; v < 10; v++ {
				if len(digits[v]) <= 0 {
					continue
				}
				i := digits[v][0]
				c := counter.Count(0, i-1)
				if c <= k {
					digits[v] = digits[v][1:]
					counter.Update(i)
					return i, c
				}
			}
			return -1, -1
		}()
		if i < 0 {
			break
		}
		rv[p] = num[i]
		p++
		used[i] = true
		k = k - c
	}
	for i := 0; i < n; i++ {
		if !used[i] {
			rv[p] = num[i]
			p++
		}
	}

	return string(rv)
}

// Counter use segment tree to calculate digits between
type Counter struct {
	n   int
	stc []int
}

// MakeCounter construct a segment tree structure
func MakeCounter(n int) Counter {
	stc := make([]int, 2*int(math.Pow(2, math.Ceil(math.Log2(float64(n)))))-1)
	var constructST func(i, b, e int) int
	constructST = func(i, b, e int) int {
		if b == e {
			stc[i] = 1
		} else {
			m := b + (e-b)/2
			stc[i] = constructST(i*2+1, b, m) + constructST(i*2+2, m+1, e)
		}
		return stc[i]
	}
	constructST(0, 0, n-1)
	return Counter{n, stc}
}

// Count returns the number of digits between qb and qe
func (c Counter) Count(qb, qe int) int {
	var countST func(i, b, e int) int
	countST = func(i, b, e int) int {
		if qb <= b && qe >= e {
			return c.stc[i]
		}
		if qb > e || qe < b {
			return 0
		}
		m := b + (e-b)/2
		return countST(i*2+1, b, m) + countST(i*2+2, m+1, e)
	}
	return countST(0, 0, c.n-1)
}

// Update remove digit at position p and update the data
func (c Counter) Update(p int) {
	var updateST func(i, b, e int) int
	updateST = func(i, b, e int) int {
		if p < b || p > e {
			return c.stc[i]
		}
		if b == e {
			if p == b {
				c.stc[i] = 0
			}
			return c.stc[i]
		}
		m := b + (e-b)/2
		c.stc[i] = updateST(i*2+1, b, m) + updateST(i*2+2, m+1, e)
		return c.stc[i]
	}

	updateST(0, 0, c.n-1)
}

//

func minIntegerSimple(num string, k int) string {
	numBytes := []byte(num)
	length := len(numBytes)

	for b := 0; b < length && k > 0; b++ {
		i := func() int {
			if numBytes[b] == '0' {
				return b
			}
			minV := numBytes[b]
			minI := b
			for i := b + 1; i < length && i <= b+k; i++ {
				if numBytes[i] < minV {
					minV = numBytes[i]
					minI = i
				}
				if minV == '0' {
					break
				}
			}
			return minI
		}()
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
