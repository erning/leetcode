#include <stdio.h>
#include <stdlib.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* getRow(int rowIndex, int* returnSize){
    *returnSize = rowIndex + 1;
    int* row = calloc(*returnSize, sizeof(int));
    row[0] = 1;
    for (int i = 1; i < *returnSize; i++) {
        for (int j = i; j > 0; j--) {
            row[j] = row[j-1] + row[j];
        }
    }
    return row;
}

int main(void) {
    int returnSize;
    int* rv = getRow(6, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d,", rv[i]);
    }
    printf("\n");
}
