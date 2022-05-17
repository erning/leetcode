package solution

import (
	"fmt"
	"testing"
)

func TestSolveNQueens(t *testing.T) {
	ret := solveNQueens(8)
	fmt.Printf("%v\n", ret)
}
