// https://www.youtube.com/watch?v=zuegQmMdy8M

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void run_1(void);
void run_2(void);
void run_3(void);
void run_4(void);
void run_5(void);

int main() {
    run_1();
    printf("-----\n");
    run_2();
    printf("-----\n");
    run_3();
    printf("-----\n");
    run_4();
    printf("-----\n");
    run_5();
    return 0;
}

// -------------------------
// multi-dimensional arrays
void run_1(void) {
    {
        // adress : 200  204  208  212
        // value  :  2    4    6    8
        //         a[0] a[1] a[2] a[3]

        int a[5] = {2, 4, 6, 8};
        int *p = a;
        printf("%d %d %d\n", p, *p, *(p + 2)); // 200, 2, 6
        p = a;
        // a = p; // error
        printf("-----\n");
    }
    {
        // adress :  400                      412             420
        // value  :   2        4       6       8       10      12
        //         b[0][0] b[0][1]          b[1][0]
        //         b[0]                     b[1]

        int b[2][3] = {{2, 4, 6}, {8, 10, 12}};
        // int *p = b; // error
        printf("%d\n", b); // 400

        // pointer to 1-D array 0f 3 integers
        int (*p)[3] = b;
        printf("%d\n", p); // 400
        printf("%d %d %d %d\n", *b, &b[0], b[0], &b[0][0]); // all 400

        printf("%d %d %d %d %d\n", b + 1, &b[1], b[1], *(b + 1), &b[1][0]); // all 412

        // b[1], *(b + 1) -> return int * (sizeof 4 bytes)
        printf("%d ", *(b + 1) + 2); // 412 + 4*2 = 420
        printf("%d %d\n", b[1] + 2, &b[1][2]); // 420

        //     b        : int (*)[3]
        // -> *b = b[0] : int *                  400
        // -> *b + 1    : (+ 4bytes) = &b[0][1]  404
        // -> *(*b + 1) :  4         = b[0][1]    4
        printf("%d %d\n", *(*b + 1), *(&b[0][1]));
        printf("%d\n", b[1][2]);
    }

    // for 2-D array
    // a[i][j] = *(a[i] + j)
    //         = *(*(a + i) + j)
}

// -------------------------
// 3-D array
void run_2(void) {
    // adress :  800          816           832
    // value  :   2  4  6  8  10            18
    //         a[0][0][0]     a[1][0][0]    a[2][0][0]
    //         a[0][0]        a[1][0]       a[2][0]
    //         a[0]           a[1]          a[2]
    int a[3][2][2] = {{{2, 4}, {6, 8}},
                    {{10, 12}, {14, 16}},
                    {{18, 20}, {22, 24}}};

    // pointer to 'int (*)[2][2]'
    int (*p)[2][2] = a;
    printf("%d\n", p); // 800
    // return pointer to 'int (*)[2]'
    printf("%d %d %d %d\n", a, *a, a[0], &a[0][0]); // all 800

    printf("%d equal %d\n", *(a[0][1] + 1), a[0][1][1]); // 8

    printf("%d %d %d\n", *(a[1] + 1), a[1][1], &a[1][1][0]); // 824

    // a[i][j][k] = *(a[i][j] + k)
    //            = *(*(a[i] + j) + k)
    //            = *(*(*(a + i) + j) + k)
}

// -------------------------
// 1-D array of integers
void func3_1(int *a) { // or a[]
    return;
}

// 2-D array of integers
void func3_2(int (*b)[3]) { // or b[][3] or **b
    return;
}

// 3-D array of integers
void func3_3(int (*c)[2][2]) { // or c[][2][2] or ***c
    return;
}

void run_3(void) {
    int a[2] = {1, 2};
    func3_1(a);

    int b[2][3] = {{2, 4, 6}, {8, 10, 12}};
    func3_2(b);

    int c[3][2][2] = {{{2, 4}, {6, 8}},
                    {{10, 12}, {14, 16}},
                    {{18, 20}, {22, 24}}};
    func3_3(c);
}

// -------------------------
// dynamic memory
/*
    Application's memory
    heap : free store (not heap data-structure)
    stack : function calls, local variables
    static/global : global
    code(text) : instructions

    c : (functions) malloc, calloc, realloc, free
    c++ : (operators) new, delete
*/
void run_4() {
    int a; // goes on stack
    int *p;
    // reserved and allocated 4bytes on the heap, return pointer starting adress
    // (stack)p stores (heap)start adress
    p = (int*)malloc(sizeof(int));
    *p = 10;

    // if not this, 10 remain on heap
    free(p);

    p = (int*)malloc(sizeof(int) * 5);
    *p = 20;
}

// -------------------------
/*
    Allocate block of memory
    - malloc : void* malloc(size_t size)  // size_t : unsigned int
    - calloc : void* calloc(size_t num, size_t size)
    - realloc : void* realloc(void* ptr, size_t size)
    Delete block of memory
    - free
*/
void run_5() {
    {
        void *p = malloc(4);
        printf("%d\n", p);
        // p += 1; // can't. void
        free(p);
    }
    {
        // typecasting
        int *p = (int*)malloc(3 * sizeof(int));
        *p = 5;
        *(p + 1) = 8;
        p[2] = 67;
        printf("%d %d %d %d\n", p, p[0], p[1], p[2]);
        free(p);
    }
    // dynamically allocated array
    {
        int n;
        printf("Enter the size of array\n");
        scanf("%d", &n);
        int *a = (int*)malloc(n * sizeof(int));
        // need initialization
        for (int i = 0; i < n; i++) {
            a[i] = i + 1;
        }
        for (int i = 0; i < n; i++) {
            printf("%d ", a[i]);
        }
        printf("%\n");
        free(a);
    }
    {
        int n;
        printf("Enter the size of array\n");
        scanf("%d", &n);
        int *a = (int*)calloc(n, sizeof(int));
        // doesn't need initialization, all 0
        for (int i = 0; i < n; i++) {
            printf("%d ", a[i]);
        }
        printf("%\n");
        free(a);

        a[2] = 5;
        for (int i = 0; i < n; i++) {
            printf("%d ", a[i]);
        }
        free(a);
    }
    {
        int n;
        printf("Enter the size of array\n");
        scanf("%d", &n);
        int *a = (int*)malloc(n * sizeof(int));
        for (int i = 0; i < n; i++) {
            a[i] = i + 1;
        }

        // equivalent to free(a)
        int *b = (int*)realloc(a, 2 * n * sizeof(int));
        printf("%d %d\n", a, b);
        for (int i = 0; i < 2 * n; i++) {
            printf("%d ", b[i]);
        }
        printf("%\n");
        free(a);

        // same as malloc
        // int *a = (int*)realloc(NULL, n * sizeof(int));
    }
}
