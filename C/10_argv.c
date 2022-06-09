#include <stdio.h>
#include <string.h>

// command-line argument, argv[0] = ./xxx
int main(int argc, char *argv[]) {
    // full path
    printf("%s\n", argv[0]);

    // argv
    printf("%s\n", argv[1]);

    if (argc == 2) {
        for (int i = 0, n = strlen(argv[1]); i < n; i++) {
            printf("%c", argv[1][i]);
        }
    }
    printf("\n");

    return 0;
}
