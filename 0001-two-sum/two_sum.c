#include <stdlib.h>

const int HTSIZE = 10000;

typedef struct node_t {
    int v;
    int p;
    struct node_t* next;
} node_t;

static inline int ht_hash(int v) {
    return (v > 0 ? v : -v) % HTSIZE;
}

int ht_find(node_t** ht, int v) {
    int k = ht_hash(v);
    node_t* n = ht[k];
    while (n) {
        if (n->v == v) return n->p;
        n = n->next;
    }
    return -1;
}

void ht_insert(node_t** ht, int v, int p) {
    int k = ht_hash(v);
    node_t* nn = malloc(sizeof(node_t));
    nn->v = v;
    nn->p = p;
    nn->next = NULL;
    node_t* n = ht[k];
    if (!n) {
        ht[k] = nn;
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
    node_t** ht= calloc(HTSIZE, sizeof(node_t*));
    for (int i = 0; i < numsSize; i++) {
        int a = nums[i];
        int b = target - a;
        int j = ht_find(ht, b);
        if (j >= 0) {
            int* rv = malloc(sizeof(int) * 2);
            if (!rv) return NULL;
            *returnSize = 2;
            rv[0] = j;
            rv[1] = i;
            return rv;
        }
        ht_insert(ht, a, i);
    }
    return NULL;
}

int main(void) {
    int nums[] = {2,7,11,15};
    int returnSize = 0;
    int* rv = twoSum(nums, 4, 9, &returnSize);
}
