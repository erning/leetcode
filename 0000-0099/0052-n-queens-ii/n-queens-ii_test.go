package solution

import (
	"fmt"
	"testing"
)

func TestTotalNQueens(t *testing.T) {
	ret := totalNQueens(8)
	fmt.Printf("%v\n", ret)
}
