package solution

import (
	"bytes"
)

func longestPrefix(s string) string {
	return longestPrefixKMP(s)
}

// [Knuth–Morris–Pratt algorithm](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm)
// https://leetcode.com/problems/longest-happy-prefix/discuss/547446/C%2B%2BJava-with-picture-incremental-hash-and-KMP
func longestPrefixKMP(s string) string {
	b := []byte(s)
	dp := make([]int, len(b))
	j := 0
	for i := 1; i < len(b); i++ {
		if b[i] == b[j] {
			j++
			dp[i] = j
		} else if j > 0 {
			j = dp[j-1]
			i--
		}
	}

	return string(b[:j])
}

func longestPrefixBruteForce(s string) string {
	b := []byte(s)
	for i := 1; i < len(b); i++ {
		if b[0] != b[i] {
			continue
		}
		if bytes.Equal(b[:len(b)-i], b[i:]) {
			return string(b[i:])
		}
	}
	return ""
}
