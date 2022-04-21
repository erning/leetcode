#include <stdlib.h>

struct Node {
    int val;
    struct Node *next;
    struct Node *random;
};

struct Node* copyRandomList(struct Node* head) {
    struct Node root = { 0, NULL, NULL };
    struct Node* prev = &root;

    struct Node* p = head;
    while (p) {
        struct Node* node = malloc(sizeof(struct Node));
        node->val = p->val;
        node->next = NULL;
        node->random = NULL;
        prev->next = node;
        p = p->next;
        prev = node;
    }

    struct Node* q = root.next;
    p = head;
    while (p) {
        if (p->random) {
            struct Node* i = head;
            struct Node* j = root.next;
            while (i) {
                if (p->random == i) {
                    q->random = j;
                    break;
                }
                i = i->next;
                j = j->next;
            }
        }
        p = p->next;
        q = q->next;
    }

    return root.next;
}

int main() {
    copyRandomList(NULL);
    return 0;
}
