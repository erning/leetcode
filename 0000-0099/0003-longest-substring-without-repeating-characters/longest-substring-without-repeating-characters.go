package solution

func lengthOfLongestSubstring(s string) int {
	max := 0
	m := [256]int{}

	slen := len(s)
	p := 0
	for i := 0; i < slen; i++ {
		if m[s[i]] >= p {
			p = m[s[i]]
		}
		m[s[i]] = i + 1
		if length := i - p + 1; length > max {
			max = length
		}
	}

	return max
}
