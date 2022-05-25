#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int* heap;
    // malloc : return pointer of void
    heap = (int*)malloc(4 * 10);

    if (heap == NULL) {
        exit(0);
    }

    for (int i = 0; i < 10; i++) {
        heap[i] = i;
    }
    for (int i = 0; i < 10; i++) {
        printf("%d ", heap[i]);
    }
    printf("\n");

    // expand memory space
    heap = (int*)realloc(heap, 4 * 100);

    free(heap);

    return 0;
}