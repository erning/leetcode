package solution

import (
	"encoding/json"
	"io/ioutil"
	"testing"
)

func TestMaxValueOfEquation(t *testing.T) {
	tf := func(points [][]int, k int, expected int) {
		output := findMaxValueOfEquation(points, k)
		if output != expected {
			t.Errorf("input: \"%v k=%d\", expected: \"%v\", output: \"%v\"\n",
				points, k, expected, output)
		}
	}

	tf([][]int{{1, 3}, {2, 0}, {5, 10}, {6, -10}}, 1, 4)
	tf([][]int{{0, 0}, {3, 0}, {9, 2}}, 3, 3)

	{
		file, _ := ioutil.ReadFile("case-63.json")
		data := [][]int{}
		_ = json.Unmarshal([]byte(file), &data)
		tf(data, 200000000, 399134490)
	}
}
