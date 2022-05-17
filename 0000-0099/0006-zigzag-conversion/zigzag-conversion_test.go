package solution

import "testing"

func TestFindMedianSortedArrays(t *testing.T) {
	tf := func(s string, numRows int, expected string) {
		output := convert(s, numRows)

		t.Logf("expected: \"%v\", output:\"%v\"\n", expected, output)
		if output != expected {
			t.Errorf("s=\"%s\", numRows=%d\n", s, numRows)
			t.FailNow()
		}
	}

	tf("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR")
	tf("PAYPALISHIRING", 4, "PINALSIGYAHRPI")

	tf("PAYPALISHIRING", 1, "PAYPALISHIRING")
	tf("PAYPALISHIRING", 2, "PYAIHRNAPLSIIG")
	tf("PAYPALISHIRING", 100, "PAYPALISHIRING")
}
