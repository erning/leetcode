package solution

import (
	"testing"
)

func TestMinInteger(t *testing.T) {
	tf := func(num string, k int, expected string) {
		output := minInteger(num, k)
		if output != expected {
			t.Errorf("input: \"%v\", expected: \"%v\", output: \"%v\"\n",
				num, expected, output)
		}
	}

	tf("4321", 4, "1342")
	tf("100", 1, "010")
	tf("36789", 1000, "36789")
	tf("22", 22, "22")
	tf("9438957234785635408", 23, "0345989723478563548")

	tf("3142", 4, "1234")
	tf("294984148179", 11, "124498948179")
	tf("9989150892151", 5, "1989950892151")
	tf("412465599017575959104005", 22, "011244556979575959104005")
}
