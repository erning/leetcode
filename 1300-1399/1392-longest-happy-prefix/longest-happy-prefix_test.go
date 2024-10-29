package solution

import "testing"

func TestLongestPrefix(t *testing.T) {
	tf := func(input, expected string) {
		output := longestPrefix(input)
		if output != expected {
			t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
				input, expected, output)
		}
	}

	tf("level", "l")
	tf("ababab", "abab")
	tf("leetcodeleet", "leet")
	tf("a", "")

	tf("", "")
	tf("acccbaaacccbaac", "ac")
}
