#include <stdio.h>
#include <ctype.h>
#include <string.h>

int main(void) {
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
