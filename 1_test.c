#include <stdio.h>

int main() {
    char s1 = 'A';
    char s2 = 'B';
    char s3 = 'C';
    printf("%c%c%c\n", s1, s2, s3);

    char *hello = "Hello";
    char name[] = "habvi";
    printf("%s %s\n", hello, name);

    int num = 5;
    printf("%d\n", num + 10);
    printf("%f\n", num + 10.0);
    printf("%.12f\n", num + 10.0);

    printf("%%c : char\n%%s : char *\n%%d : integer\n");
    printf("%%f : float double\n%%ld : long\n%%lf : long double\n");

    const int NUM = 4;
    printf("%d\n", NUM);

    double d = 1.23456;
    int num2 = 1234;
    printf("zero fill f : %010f\n", d);
    printf("zero fill d : %010d\n", num2);

    printf("%f\n", 5 / 4);
    printf("%d\n", 10 / 4);

    char c = '#';
    // same
    printf("%i\n", (int) c);
    printf("%i\n", c);

    return 0;
}