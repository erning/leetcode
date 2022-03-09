#include <stdlib.h>

typedef struct node_t {
    int v;
    int p;
    struct node_t* next;
} node_t;

int find(node_t** hash, int b) {
    int k = (b > 0 ? b : -b) % 1000;
    node_t* n = hash[k];
    while (n) {
        if (n->v == b) return n->p;
        n = n->next;
    }
    return -1;
}

void insert(node_t** hash, int v, int p) {
    node_t* nn = malloc(sizeof(node_t));
    nn->v = v;
    nn->p = p;
    nn->next = NULL;
    int k = (v > 0 ? v : -v) % 1000;
    node_t* n = hash[k];
    if (!n) {
        hash[k] = nn;
        return;
    }
    while (n->next) {
        n = n->next;
    }
    n->next = nn;
}

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    *returnSize = 0;
    node_t** hash = calloc(1000, sizeof(node_t*));
    for (int i = 0; i < numsSize; i++) {
        int a = nums[i];
        int b = target - a;
        int j = find(hash, b);
        if (j >= 0) {
            int* rv = malloc(sizeof(int) * 2);
            if (!rv) return NULL;
            *returnSize = 2;
            rv[0] = i;
            rv[1] = j;
            return rv;
        }
        insert(hash, a, i);
    }
    return NULL;
}

int main(void) {
    int nums[] = {2,7,11,15};
    int returnSize = 0;
    int* rv = twoSum(nums, 5, 9, &returnSize);
}
