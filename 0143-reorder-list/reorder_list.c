#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

void print_list(struct ListNode* node) {
    while (node) {
        printf("%d, ", node->val);
        node = node->next;
    }
    printf("\n");
}

struct ListNodeRev {
    struct ListNode *node;
    struct ListNodeRev *prev;
};

void reorderList(struct ListNode* head) {
    struct ListNode* node = head;
    struct ListNodeRev *r_node = NULL;
    while (node) {
        struct ListNodeRev* r_next = malloc(sizeof(struct ListNodeRev));
        r_next->node = node;
        r_next->prev = r_node;
        node = node->next;
        r_node = r_next;
    }
    node = head;
    while (node->next && node->next != r_node->node) {
        r_node->prev->node->next = NULL;
        r_node->node->next = node->next;
        node->next = r_node->node;

        node = r_node->node->next;
        r_node = r_node->prev;
    }
}

int main(void) {
    struct ListNode a1;
    struct ListNode a2;
    struct ListNode a3;
    struct ListNode a4;
    struct ListNode a5;

    a1.val = 1;
    a1.next = &a2;
    a2.val = 2;
    a2.next = &a3;
    a3.val = 3;
    a3.next = &a4;
    a4.val = 4;
    a4.next = &a5;
    a5.val = 5;
    a5.next = NULL;

    reorderList(&a1);
    print_list(&a1);
}
