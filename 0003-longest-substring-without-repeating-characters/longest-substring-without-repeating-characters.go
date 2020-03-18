package solution

func lengthOfLongestSubstring(s string) int {
	max := 0
	m := [256]int{}

	for i, p, slen := 0, 0, len(s); i < slen && p < slen-max; i++ {
		v := s[i]
		j := m[v] - 1
		if j < 0 {
			if length := i - p + 1; length > max {
				max = length
			}
			m[v] = i + 1
			continue
		}

		for p <= j {
			m[s[p]] = 0
			p++
		}
		m[v] = i + 1
	}

	return max
}
