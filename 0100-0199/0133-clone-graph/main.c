#include <stdlib.h>
#include <stdio.h>

struct Node {
    int val;
    int numNeighbors;
    struct Node** neighbors;
};

struct Node *clone_graph(struct Node *s, struct Node** visited) {
    if (s == NULL) return NULL;
    if (visited[s->val]) return visited[s->val];

    struct Node* node = malloc(sizeof(struct Node));
    visited[s->val] = node;

    node->val = s->val;
    node->numNeighbors = s->numNeighbors;
    node->neighbors = malloc(sizeof(struct Node*) * node->numNeighbors);
    for (int i = 0; i < s->numNeighbors; ++i) {
        node->neighbors[i] = clone_graph(s->neighbors[i], visited);
    }
    return node;
}

struct Node *cloneGraph(struct Node *s) {
    struct Node** visited = calloc(101, sizeof(struct Node*));
    return clone_graph(s, visited);
}

int main() {
    cloneGraph(NULL);
    return 0;
}

