package solution

import "testing"

func TestLongestPalindrome(t *testing.T) {
	tf := func(input, expected string) {
		output := longestPalindrome(input)

		t.Logf("expected:\"%v\", output:\"%v\"", expected, output)
		if output != expected {
			t.Error()
		}
	}

	tf("", "")
	tf("1", "1")
	tf("12321", "12321")
	tf("1122333", "333")
	tf("121121", "121121")
	tf("babad", "bab") // or aba
	tf("cbbd", "bb")
}
