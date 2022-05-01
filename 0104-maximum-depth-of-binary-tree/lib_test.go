package solution

import (
	"testing"
)

func TestMaxDepth(t *testing.T) {
	output := maxDepth(nil)
	if output != 0 {
		t.Error()
	}
}
