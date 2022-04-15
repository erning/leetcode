
// Definition for a binary tree node.
type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func maxDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    d1 := 1 + maxDepth(root.Left)
    d2 := 1 + maxDepth(root.Right)
    if d1 > d2 {
        return d1
    }
    return d2
}
