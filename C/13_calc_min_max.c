#include <stdio.h>

void line(void);

void calc_min_max(int arr[], int *min, int *max) {
    *min = 100;
    *max = 0;
    int i = 0;
    while (arr[i] != -1) {
        if (arr[i] < *min) {
            *min = arr[i];
        }
        if (arr[i] > *max) {
            *max = arr[i];
        }
        i++;
    }
}

int main() {
    int arr[] = {3, 6, 2, 35, 5, -1};
    int min, max;
    calc_min_max(arr, &min, &max);
    printf("min: %d, max: %d", min, max);
    return 0;
}

void line(void) {
    printf("-----\n");
}
