#include <stdio.h>
#include <string.h>

// wanna erase array[] ..
float average(int, int array[]);


int main() {
    int num[] = {1, 2, 4, 6, 8};
    num[3] = 100;
    printf("%d\n", num[3]);

    int num2[10];
    num2[1] = 5;
    // undefined
    printf("%d\n", num2[0]); 
    printf("%d\n", num2[1]);

    // array of char
    char s1[6] = "hi!";
    printf("%s\n", s1);

    printf("%c%c%c\n", s1[0], s1[1], s1[2]);
    // tail of s1 : there is null = '\0' = 0
    printf("%i %i %i %i\n", s1[0], s1[1], s1[2], s1[3]);

    // >> hellohi! .. ???
    char s2[5] = "hello";
    for (int i = 0; s2[i] != '\0'; i++) {
        printf("%c", s2[i]);
    }
    printf("\n");
    // 8????
    printf("%i\n", strlen(s2));

    // for '\0'? +1 ! >> hello
    char s3[6] = "hello";
    for (int i = 0; i < strlen(s3); i++) {
        printf("%c", s3[i]);
    }
    printf("\n");

    for (int i = 0, n = strlen(s3); i < n; i++) {
        printf("%c ", s3[i]);
    }
    printf("\n");


    const int LENGTH = 3;
    int array[] = {1, 2, 4};
    printf("%.10f\n", average(LENGTH, array));

    return 0;
}

float average(int length, int array[]) {
    int sum = 0;
    for (int i = 0; i < length; i++) {
        sum += array[i];
    }

    return sum / (float) length;
}