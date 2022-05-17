package solution

// TreeNode - Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func generateTrees(n int) []*TreeNode {
	if n <= 0 {
		return nil
	}
	var generateTrees func(a, b int) []*TreeNode
	generateTrees = func(a, b int) []*TreeNode {
		if a > b {
			return nil
		}
		if a == b {
			return []*TreeNode{{a, nil, nil}}
		}
		if b-a == 1 {
			return []*TreeNode{
				{a, nil, &TreeNode{b, nil, nil}},
				{b, &TreeNode{a, nil, nil}, nil},
			}
		}
		ans := []*TreeNode{}
		for m := a; m <= b; m++ {
			left := generateTrees(a, m-1)
			right := generateTrees(m+1, b)
			if len(left) == 0 {
				for _, y := range right {
					ans = append(ans, &TreeNode{
						Val:   m,
						Left:  nil,
						Right: y,
					})
				}
				continue
			}
			if len(right) == 0 {
				for _, x := range left {
					ans = append(ans, &TreeNode{
						Val:   m,
						Left:  x,
						Right: nil,
					})
				}
				continue
			}
			for _, x := range left {
				for _, y := range right {
					ans = append(ans, &TreeNode{
						Val:   m,
						Left:  x,
						Right: y,
					})
				}
			}
		}
		return ans
	}
	return generateTrees(1, n)
}
