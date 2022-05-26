// https://www.youtube.com/watch?v=zuegQmMdy8M

#include <stdio.h>
#include <string.h>

void run_1(void);
void run_2(void);
void run_3(void);
void run_4(void);
void run_5(void);
void run_6(void);
void run_7(void);
void run_8(void);
void run_9(void);

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
    printf("-----\n");
    run_6();
    printf("-----\n");
    run_7();
    printf("-----\n");
    run_8();
    printf("-----\n");
    run_9();
    return 0;
}

void run_1(void) {
    // ex) a's adress : 204, p's adress : 64

    int a = 5;
    printf("%d\n", &a);  // 204

    // pointer to integer
    int *p;
    printf("%d\n", p);  // undefined

    // adress of a
    p = &a;
    printf("%d\n", p);   // 204
    printf("%d\n", *p);  // 5  dereference, value at adress pointed by p
    printf("%d\n", &p);  // 64

    a = 10;
    printf("%d %d\n", p, *p);   // 204, 10
    *p = 12;
    printf("%d\n", a);   // 12

    int b = 30;
    *p = b;
    printf("%d %d\n", p, &p);   // 204, 64
    printf("%d %d\n", a, *p);   // 30, 30
    printf("-----\n");

    // pointer arithmetic
    printf("%d\n", p);       // 204
    printf("%d\n", p + 1);   // 208
    printf("Because size of integer is %d bytes\n", sizeof(int));

    printf("print  p : value = 204\n");
    printf("print *p : look at 4 bytes starting 204\n");
}

void run_2(void) {
    {
        int a = 5;
        int *p = &a;
        printf("%d %d %d\n", p, *p, *(p + 1)); // 5, undifined

        int b = 10;
        *p = b;
        printf("%d %d %d\n", p, *p, *(p + 1)); // 5, undefined
    }
    {
        int a = 5;
        int *p = &a;
        printf("%d %d %d\n", p, *p, *(p + 1)); // 5, 10 : because not freed up the memory

        int b = 30;
        *p = b;
        printf("%d %d %d\n", p, *p, *(p + 1)); // 30, 10 : because not freed up the memory
    }
    return;
}

// pointer to integer
void run_3(void) {
    // 1025 = 00000000 00000000 00000100 00000001
    //           203      202      201      200   <- adress
    int a = 1025;
    int *p = &a;
    printf("%d %d %d\n", sizeof(int), p, *p);
    printf("%d %d %d\n", sizeof(int), p + 1, *(p + 1)); // +4, undefined

    // typecasting
    char *pc = (char*)p;
    printf("%d %d %d\n", sizeof(char), pc, *pc); // 200
    printf("%d %d %d\n", sizeof(char), pc + 1, *(pc + 1)); // +1, 201

    // void pointer - generic pointer
    void *pv;
    pv = p;
    printf("%d\n", pv);
    printf("%d\n", pv + 1); // error...?
    // printf("%d\n", *pv); // error
    return;
}

// pointer to pointer
void run_4(void) {
    int x = 5;
    int *p = &x;
    int **p2 = &p;
    int ***p3 = &p2;
    // *(*p2) = **p2
    printf("%d %d %d %d %d %d\n", p, p2, p3, *p, **p2, ***p3);
    ***p3 = 8;
    printf("%d ", x);
    **p2 = *p + 10;
    printf("%d\n", x);
}

// -------------------------
void func_5(int *p) {
    *p += 10;
}

void run_5(void) {
    int a = 5;
    printf("%d ", a);
    func_5(&a);
    printf("%d\n", a);
}

// -------------------------
// array
void run_6(void) {
    int a[5] = {1, 2, 3, 4, 5};
    int *p = a;
    // a = p = a's head pointer
    printf("%d %d %d %d\n", a, *a, p, *p);
    printf("%d %d %d %d\n", a + 1, *(a + 1), p + 1, *(p + 1)); // +4 bytes

    // adress
    printf("%d %d ", &a[3], a + 3);
    // value
    printf("%d %d\n", a[3], *(a + 3));

}

// -------------------------
// array always passed to func by reference
// same : int *a, int a[]
int func_7(int *a, int size) {
    printf("%d ", sizeof(a));
    int total = 0;
    for(int i = 0; i < size; i++) {
        total += a[i];
        a[i] *= 2;
    }
    return total;
}

void run_7(void) {
    int a[] = {1, 2, 3, 4, 5};
    int size = sizeof(a) / sizeof(a[0]);
    int total = func_7(a, size);
    printf("%d\n", total);

    for (int i = 0; i < size; i++) {
        printf("%d ", a[i]);
    }
    printf("\n");
}

// -------------------------
// all same
void func_8(char *s) {
    char *tmp = s;
    while (*s) {
        printf("%c", *s);
        s++;
    }
    printf("\n");

    s = tmp;
    int i = 0;
    while (s[i] != '\0') {
        printf("%c", s[i]);
        i++;
    }
    printf("\n");

    s = tmp;
    i = 0;
    while (*(s + i) != '\0') {
        printf("%c", *(s + i));
        s++;
    }
    printf("\n");
}

// caracter arrays (like string)
void run_8(void) {
    {
        char s[4];
        s[0] = 'j';
        s[1] = 'o';
        s[2] = 'h';
        s[3] = 'n';
        s[4] = '\0';
        printf("%s %d\n", s, strlen(s));
    }
    {
        char s[] = "alice";
        printf("%s %d\n", s, strlen(s));
        char *pc = s;
        printf("%d %c%c%c%c%c\n", pc, *pc, pc[1], pc[2], pc[3], pc[4]);
        pc += 4;
        printf("%c\n", *pc);

        func_8(s);
    }
}

// -------------------------
void run_9(void) {
    // gets stored in the space for array
    char s[10] = "hello";
    printf("%s\n", s);

    // gets stored as compile time constant
    char *a = "abcd";
    printf("%s\n", a);
    a[1] = 'z'; // Segmentation fault
    printf("%s\n", a);
}