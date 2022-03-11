struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

#include <stdlib.h>

void traverse(int* output, int *returnSize, struct TreeNode* node) {
    if (!node) return;
    output[*returnSize] = node->val;
    (*returnSize)++;
    traverse(output, returnSize, node->left);
    traverse(output, returnSize, node->right);
}

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* preorderTraversal(struct TreeNode* root, int* returnSize){
    int* output = malloc(sizeof(int) * 100);
    *returnSize = 0;
    traverse(output, returnSize, root);
    return output;
}

int main(void) {
    return 0;
}
