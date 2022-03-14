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

struct ListNode* reverse(struct ListNode* node) {
    struct ListNode* prev = NULL;
    struct ListNode* curr = node;
    struct ListNode* next = NULL;
    while (curr != NULL) {
        next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }
    node = prev;
    return node;
}

void reorderList(struct ListNode* head) {
    if (!head || !head->next || !head->next->next) {
        return;
    }

    // Devide the list in two parts from the middle.
    struct ListNode* fast = head;
    struct ListNode* slow = head;
    struct ListNode* prev = head;
    while (fast && fast->next) {
        prev = slow;
        slow = slow->next;
        fast = fast->next->next;
    }
    prev->next = NULL;

    // Reverse the second half.
    struct ListNode* tail = reverse(slow);

    // Merge the two halfs.
    while (1) {
        struct ListNode* next = head->next;
        struct ListNode* prev = tail->next;
        head->next = tail;
        if (!next) {
            break;
        }
        tail->next = next;
        head = next;
        tail = prev;
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
