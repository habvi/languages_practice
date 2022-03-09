#include <stdio.h>

int main() {
    int num[] = {1, 2, 4, 6, 8};
    num[3] = 100;
    printf("%d\n", num[3]);

    int num2[10];
    num2[1] = 5;
    // undefined
    printf("%d\n", num2[0]); 
    printf("%d\n", num2[1]);

    // charのlistでstringということだったらしい
    char list[20] = "array";
    printf("%s", list);

    return 0;
}