#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char c = 'A';
    printf("%c ", c);

    c = 'A' + 3;
    printf("%c ", c);

    // char -> int
    c = '8';
    if ('0' <= c && c <= '9') {
        int i = c - '0';
        printf("%d ", i);
    } else {
        printf("not int..");
    }

    // same (<ctype.h> : isalpha, isupper, islower...)
    if (isdigit(c)) {
        int i = c - '0';
        printf("%d\n", i);
    } else {
        printf("not int..");
    }


    // <stdlib.h>
    char str[] = "235";
    char str2[] = "-235";
    char str3[] = "abcd";   // -> 0
    int i = atoi(str);
    int i2 = atoi(str2);
    int i3 = atoi(str3);
    printf("%d %d %d\n", i, i2, i3);

    // copy
    {
    char str[] = "abcde";
    char str2[6];
    for (i = 0; i < sizeof(str) / sizeof(str[0]); i++) {
        str2[i] = str[i];
    }
    printf("%s %s\n", str, str2);

    // <string.h>
    char str3[6];
    strcpy(str3, "hello");
    printf("%s\n", str3);
    }


    return 0;
}
