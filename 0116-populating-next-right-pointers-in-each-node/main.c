#include <stdlib.h>
#include <stdio.h>

/**
 * Definition for a Node.
 */
struct Node {
    int val;
    struct Node *left;
    struct Node *right;
    struct Node *next;
};

struct Node *nodes[12];

void traversal(struct Node* node, int depth) {
    if (node == NULL) {
        return;
    }
    struct Node *prev = nodes[depth];
    if (prev != NULL) {
        prev->next = node;
    }
    nodes[depth] = node;
    traversal(node->left, depth + 1);
    traversal(node->right, depth + 1);
}

struct Node* connect(struct Node* root) {
    traversal(root, 0);
    return root;
}

int main() {
    connect(NULL);
    return 0;
}

