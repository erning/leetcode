package solution

import "testing"

func TestTwoSum(t *testing.T) {
	tf := func(input string, expected int) {
		output := lengthOfLongestSubstring(input)
		t.Logf("input: \"%v\", expected: %v, output: %v", input, expected, output)
		if output != expected {
			t.Error()
		}
	}

	tf("abcabcbb", 3)
	tf("bbbbb", 1)
	tf("pwwkew", 3)

	tf("", 0)
	tf("a", 1)
	tf("ab", 2)

	tf("aab", 2)
}
