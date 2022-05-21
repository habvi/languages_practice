#include <stdio.h>

int main() {
    int num;
    printf("Enter the integer : ");
    // & : referencing an address. need int or char.
    scanf("%d", &num);
    printf("The number is .. %d\n", num);


    char name[20];
    printf("Enter your name : ");
    // till space. no need &.
    scanf("%20s", name);
    printf("Your name is .. %s\n", name);

    // up to specified number, even if spaces.
    fgets(name, 20, stdin);
    printf("Your name is .. %s\n", name);

    return 0;
}