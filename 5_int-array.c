#include <stdio.h>
#include <string.h>

float average(int, int *format);


int main() {
    int num[] = {1, 2, 4, 6, 8};
    num[3] = 100;
    printf("%d\n", num[3]);
    printf("%d\n", sizeof(num) / sizeof(num[0]));

    int num2[10];
    num2[1] = 5;
    // undefined
    printf("%d\n", num2[0]); 
    printf("%d\n", num2[1]);

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