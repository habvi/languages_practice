#include <stdio.h>
#include <ctype.h>
#include <string.h>

int main(void) {
    // array of char
    char s1[4] = "hi!";
    printf("%s\n", s1);
    printf("%c%c%c\n", s1[0], s1[1], s1[2]);
    // EOS(End of String) : there is null = '\0' = 0
    printf("%i %i %i %i\n", s1[0], s1[1], s1[2], s1[3]);

    // >> hello
    char s2[6] = "hello";
    for (int i = 0; s2[i] != '\0'; i++) {
        printf("%c", s2[i]);
    }
    printf("\n");
    printf("%i\n", strlen(s2));

    // same
    char s3[6] = "hello";
    for (int i = 0; i < strlen(s3); i++) {
        printf("%c", s3[i]);
    }
    printf("\n");

    for (int i = 0, n = strlen(s3); i < n; i++) {
        printf("%c ", s3[i]);
    }
    printf("\n");


    char *words[3];
    words[0] = "hello";
    words[1] = "world";
    printf("%c\n", words[0][0]);
    printf("%c\n", words[1][2]);

    char s1[8] = "hello";
    for (int i = 0, n = strlen(s1); i < n; i++) {
        if ('a' <= s1[i] && s1[i] <= 'z') {
            printf("%c", s1[i] - 32);
        } else {
            printf("%c", s1[i]);
        }
    }
    printf("\n");

    for (int i = 0, n = strlen(s1); i < n; i++) {
        if (islower(s1[i])) {
            printf("%c", toupper(s1[i]));
        } else {
            printf("%c", s1[i]);
        }

    }
    printf("\n");

    for (int i = 0, n = strlen(s1); i < n; i++) {
         printf("%c", toupper(s1[i]));
    }

    return 0;
}
