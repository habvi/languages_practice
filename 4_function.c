#include <stdio.h>

// void : nothing return
void sayHello(char name[]) {
    printf("Hello %s\n", name);
}

double cube(double num) {
    return num * num * num;
}

int bigger(int num1, int num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}

int main() {
    sayHello("habvi");

    printf("%f\n", cube(3));

    printf("%d", bigger(10, 22));

    return 0;
}
