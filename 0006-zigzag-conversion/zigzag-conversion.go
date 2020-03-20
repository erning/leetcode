package solution

func convert(s string, numRows int) string {
	if numRows <= 1 {
		return s
	}
	slen := len(s)
	if numRows >= slen {
		return s
	}
	b := make([]byte, slen, slen)

	var r int // rows
	var p int // position

	max := 2*numRows - 2
	m := 0

	for i := 0; i < slen; i++ {
		b[i] = s[p]

		m = max - m
		if m == 0 {
			m = max
		}
		p += m
		if p < slen {
			continue
		}

		r++
		p = r
		m = 2 * r
	}

	return string(b)
}
