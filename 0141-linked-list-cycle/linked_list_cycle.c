#include <stdbool.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

bool hasCycle(struct ListNode *head) {
    struct ListNode* p1 = head;
    struct ListNode* p2 = head;
    while (p1 && p2) {
        p1 = p1->next;
        p2 = p2->next;
        if (!p2) return false;
        p2 = p2->next;
        if (p1 == p2) return true;
    }
    return false;
}

int main(void) {

}
