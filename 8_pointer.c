#include <stdio.h>
#include <string.h>

int main() {
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
    printf("%d\n", p);   // 204
    printf("%d\n", *p);  // 10

    *p = 12;
    printf("%d\n", a);   // 12

    int b = 30;
    *p = b;
    printf("%d\n", p);   // 204
    printf("%d\n", &p);  // 64
    printf("%d\n", a);   // 30
    printf("%d\n", *p);  // 30

    // int c = 50;
    // int *p = &c;


    // pointer arithmetic
    printf("%d\n", p);       // 204
    printf("%d\n", p + 1);   // 208
    printf("Because size of integer is %d bytes\n", sizeof(int));

    printf("print  p : look at only 204\n");
    printf("print *p : look at 4 bytes starting 204");

    return 0;
}
