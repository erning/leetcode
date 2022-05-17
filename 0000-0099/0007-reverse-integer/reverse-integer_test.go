package solution

import "testing"

func TestReverse(t *testing.T) {
	tf := func(input, expected int) {
		output := reverse(input)
		if output != expected {
			t.Errorf("input: %v, expected: %v, output:%v\n",
				input, expected, output)
		}
	}

	tf(123, 321)
	tf(-123, -321)
	tf(120, 21)

	tf(0, 0)

	tf(1463847412, 2147483641)
	tf(8463847412, 0)
	tf(7463847413, 0)

	tf(-8463847412, -2147483648)
	tf(-9463847412, 0)
	tf(-8463847413, 0)
	tf(-7463847412, -2147483647)
	tf(-9463847411, -1147483649)
}
