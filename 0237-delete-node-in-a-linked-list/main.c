#include <string.h>

void deleteNode(struct ListNode* node) {
    struct ListNode* next = node->next;
    memcpy(node, next, sizeof(*node));
    free(next);
}

int main() {
    return 0;
}

