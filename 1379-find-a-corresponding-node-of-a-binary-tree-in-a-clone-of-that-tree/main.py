# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def getTargetCopy(
        self, original: TreeNode, cloned: TreeNode, target: TreeNode
    ) -> TreeNode:

        if original.val == target.val:
            return cloned

        if original.left is not None:
            v = self.getTargetCopy(original.left, cloned.left, target)
            if v is not None:
                return v

        if original.right is not None:
            v = self.getTargetCopy(original.right, cloned.right, target)
            if v is not None:
                return v

#
