#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode *getIntersectionNode(struct ListNode *headA, struct ListNode *headB) {
    struct ListNode *pa = headA;
    struct ListNode *pb = headB;
    int a = 0;
    int b = 0;
    while (pa) { a++; pa = pa->next; }
    while (pb) { b++; pb = pb->next; }
    pa = headA; pb = headB;
    while (a > b) { a--; pa = pa->next; }
    while (a < b) { b--; pb = pb->next; }
    while (pa && pb) {
        if (pa == pb) { return pa; }
        pa = pa->next;
        pb = pb->next;
    }
    return NULL;
}

int main() {
    struct ListNode b = { 1, NULL };
    struct ListNode a = { 0, &b };
    struct ListNode *x = getIntersectionNode(&b, &a);
    printf("a=%p, b=%p\n", x, &b);
    return 0;
}
