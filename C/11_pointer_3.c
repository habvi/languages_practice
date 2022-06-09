// https://www.youtube.com/watch?v=zuegQmMdy8M

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void run_1(void);
void run_2(void);
void run_3(void);
void run_4(void);
void run_5(void);
void run_6(void);

int main() {
    printf("----- 1\n");
    run_1();
    printf("----- 2\n");
    run_2();
    printf("----- 3\n");
    run_3();
    printf("----- 4\n");
    run_4();
    printf("----- 5\n");
    run_5();
    printf("----- 6\n");
    run_6();
    return 0;
}

// ------------------------------
// function pointers
// ------------------------------
// NG. local val(total) in stack memory
// int *add_ng(int *a, int *b) {
//     int total = *a + *b;
//     return &total;
// }

// heap
int *add(int *a, int *b) {
    int *total = (int*)malloc(sizeof(int));
    *total = *a + *b;
    return total;
}

void run_1(void) {
    int a = 2, b = 8;
    int *total = add(&a, &b);
    printf("%d\n", *total);
}

// ------------------------------
int add2(int a, int b) {
    return a + b;
}

void run_2(void) {
    // pointer to function
    int (*p)(int, int);
    p = add2; // or &add2

    int c;
    c = (*p)(5, 8);
    printf("%d\n", c);
    // juct p is ok
    c = p(10, 4);
    printf("%d\n", c);
}

// ------------------------------
void print(char *name) {
    printf("hello %s\n", name);
}

void run_3(void) {
    void (*p)(char *);
    p = print;
    p("Alice");
}

// ------------------------------
// function pointer and callbacks
// ------------------------------
// callback function
void f1() {
    printf("hello\n");
}

void f2(void (*p)()) {
    p();
}

void run_4() {
    f2(f1);
    // same
    // void (*p)() = f1;
    // f2(p);
}

// ------------------------------
// use flag ver.
void bubble_sort_flag(int *a, int n, int flag) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n - 1; j++) {
            // ascending
            if (flag == 0) {
                if (a[j] > a[j + 1]) {
                    int temp = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = temp;
                }
            }
            // descending
            else {
                if (a[j] < a[j + 1]) {
                    int temp = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = temp;
                }
            }
        }
    }
}

// use callback function ver.
int comp_ascending(int a, int b) {
    if (a > b) {
        return 1;
    } else {
        return -1;
    }
}
int comp_descending(int a, int b) {
    if (a > b) {
        return -1;
    } else {
        return 1;
    }
}
void bubble_sort(int *a, int n, int (*comp)(int, int)) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n - 1; j++) {
            if (comp(a[j], a[j + 1]) > 0) {
                int temp = a[j];
                a[j] = a[j + 1];
                a[j + 1] = temp;
            }
        }
    }
}

void print_array(int *a, int n) {
    for (int i = 0; i < n; i++) {
        printf("%d ", a[i]);
    }
    printf("\n");
}

void run_5() {
    // want to sort
    int a[] = {4, 2, 5, 3, 1, 6};
    int n = 6;

    bubble_sort_flag(a, n, 0);
    print_array(a, n);

    bubble_sort_flag(a, n, 1);
    print_array(a, n);

    bubble_sort(a, n, comp_ascending);
    print_array(a, n);

    bubble_sort(a, n, comp_descending);
    print_array(a, n);
}

// ------------------------------
#include <math.h>
int absolute_comp(int a, int b) {
    if (abs(a) > abs(b)) {
        return 1;
    } else {
        return -1;
    }
}

#include <stdlib.h>
int compare_ascending(const void *a, const void *b) {
    // typecasting and get value
    int c = *((int*)a);
    int d = *((int*)b);
    return c - d; // same meaning
}
int compare_descending(const void *a, const void *b) {
    int c = *((int*)a);
    int d = *((int*)b);
    return d - c;
}
int compare_absolute(const void *a, const void *b) {
    int c = *((int*)a);
    int d = *((int*)b);
    return abs(c) - abs(d);
}

void run_6(void) {
    int a[] = {8, 2, -4, 3, -10, 6};
    int n = 6;

    bubble_sort(a, n, absolute_comp);
    print_array(a, n);

    qsort(a, n, sizeof(int), compare_ascending);
    print_array(a, n);
    qsort(a, n, sizeof(int), compare_descending);
    print_array(a, n);
    qsort(a, n, sizeof(int), compare_absolute);
    print_array(a, n);
}
