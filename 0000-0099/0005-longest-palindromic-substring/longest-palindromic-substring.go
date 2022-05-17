package solution

import "fmt"

func longestPalindrome(s string) string {
	slen := len(s)
	if slen <= 0 {
		return ""
	}

	var maxLength, maxA, maxB int
	for i := 0; i < slen-maxLength/2; {
		a, b := i, i
		for b = i + 1; b < slen; b++ {
			if s[i] != s[b] {
				break
			}
		}
		c := b
		b--
		for a > 0 && b < slen-1 && s[a-1] == s[b+1] {
			a--
			b++
		}

		if maxLength < b-a {
			maxLength, maxA, maxB = b-a, a, b
		}
		fmt.Printf("i=%d s=\"%s\"\n", i, s[a:b+1])
		i = c
	}
	return s[maxA : maxB+1]
}
