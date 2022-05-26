// https://qcguide-hrd.appspot.com/15-06.html

#include <stdio.h>

void line(void);

// ------------------------------------
void func_1(int* x) {
    *x = 20;
    return;
}

void run_1(void) {
    int a = 10;
    printf("%d ", a);
    func_1(&a);
    printf("%d\n", a);
}

// ------------------------------------
// same : int data[5], int data[], int *data.
// just pass the head adress. all pointer value.
int calc_average(int data[]) {
    int average = 0;
    for (int i = 0; i < 5; i++) {
        // data[i] : head adress + i
        average += data[i];
    }
    data[0] = 0;
    return average / 5;
}

void run_2(void) {
    int average, arr[5] = {24, 1, 64, 2, 72};
    average = calc_average(arr);
    printf("%d %d ", average, arr[0]);

    // same
    int *data, average2 = 24;
    // arr's head adress
    data = arr;
    for (int i = 0; i < 5; i++) {
        average2 += data[i];
    }
    printf("%d\n", average2 / 5);
}

// ------------------------------------
void run_3(void) {
    int *data;
    int average = 0, arr[5] = {24, 1, 64, 2, 72};
    data = arr;
    for (int i = 0; i < 5; i++) {
        //  pointer arithmetic
        average += *(data + i);
    }
    printf("%d ", average / 5);

    // faster (but recently, it seems automatically compiled to the same speed.)
    average = 0;
    for (data = arr; data != &arr[5]; data++) {
        //  pointer arithmetic
        average += *data;
    }
    printf("%d", average / 5);
}

// ------------------------------------
int main() {
    {
        int a = 10, b = 15;
        printf("%d : %p, %d : %p\n", a, &a, b, &b);

        int arr[5];
        printf("%p %p %p %p\n", &arr, &arr[0], &arr[1], &arr[2]);
        printf("%d %d %d %d\n", &arr, &arr[0], &arr[1], &arr[2]);
        line();
    }
    {
        int *p;
        int i;
        p = &i;
        printf("%p %p %d %d\n", &i, p, &i, p);
        *p = 5;
        printf("%d %d\n", i, *p);
        line();
    }

    run_1();
    line();

    run_2();
    line();

    run_3();
    return 0;
}

void line(void) {
    printf("-----\n");
}
