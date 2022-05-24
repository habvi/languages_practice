#include <stdio.h>
#include <ctype.h>
#include <string.h>

void line(void) {
    printf("-----\n");
}

int main(void) {
    // array of char
    {
        char s[4] = "hi!";
        printf("%s\n", s);
        printf("%c%c%c\n", s[0], s[1], s[2]);
        // EOS(End of String) : there is null = '\0' = 0
        printf("%i %i %i %i\n", s[0], s[1], s[2], s[3]);
        line();
    }
    // >> hello
    {
        char s[6] = "hello";
        for (int i = 0; s[i] != '\0'; i++) {
            printf("%c", s[i]);
        }
        printf(" %i\n", strlen(s));

        // same
        for (int i = 0; i < strlen(s); i++) {
            printf("%c", s[i]);
        }
        printf("\n");

        // same
        for (int i = 0, n = strlen(s); i < n; i++) {
            printf("%c ", s[i]);
        }
        printf("\n");
        line();
    }
    {
        char *s[3];
        s[0] = "hello";
        s[1] = "world";
        printf("%s %c %s %c\n", s[0], s[0][0], s[1], s[1][2]);

        char s2[] = "abcd";
        printf("%s\n", s2);
        s2[1] = 'z';
        printf("%s\n", s2);
        line();
    }
    // to upper
    {
        char s[8] = "hello";
        for (int i = 0, n = strlen(s); i < n; i++) {
            if ('a' <= s[i] && s[i] <= 'z') {
                printf("%c", s[i] - 32);
            } else {
                printf("%c", s[i]);
            }
        }
        printf("\n");

        for (int i = 0, n = strlen(s); i < n; i++) {
            if (islower(s[i])) {
                printf("%c", toupper(s[i]));
            } else {
                printf("%c", s[i]);
            }

        }
        printf("\n");

        for (int i = 0, n = strlen(s); i < n; i++) {
            printf("%c", toupper(s[i]));
        }

    }
    return 0;
}
