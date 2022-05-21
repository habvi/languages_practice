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


    // <stdlib.h> chars -> int
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
    // (to, from)
    strcpy(str3, "hello");
    printf("%s ", str3);

    char str4[10];
    // specify the number of chars
    strncpy(str4, str3, 2);
    // must : add EOS
    str4[2] = '\0';
    printf("%s\n", str4);
    }


    // concatenation
    {
    char str[12] = "he""llo";
    printf("%s\n", str);

    char str2[] = " world";
    strcat(str, str2);
    printf("%s %i %i\n", str, sizeof(str), str[11]);
    }

    // compare (× str == str2)
    {
    char str[20], str2[] = "abcd";
    scanf("%s", str);

    int len = strlen(str2);
    printf("%i\n", len);

    for (i = 0; i < len + 1; ++i) {
        if (str[i] != str2[i]) {
            break;
        }
    }
    // include EOS
    if (i == len + 1) {
        printf("same\n");
    } else {
        printf("not same\n");
    }

    // same. if same, return 0
    if (strcmp(str, str2) == 0) {
        printf("same\n");
    } else {
        printf("not same\n");
    }
    }

    return 0;
}
