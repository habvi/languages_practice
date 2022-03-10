#include <stdio.h>

// prototype declaration
void sayHello(char *format);
double cube(double);
int bigger(int, int);
int countfunc(void);


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

int countfunc(void) {
    // static local variables
    static int count;
    count++;
    printf("%d\n", count);
    return count;
}


int main() {
    sayHello("habvi");

    printf("%f\n", cube(3));

    printf("%d\n", bigger(10, 22));

    countfunc();
    countfunc();
    countfunc();

    return 0;
}
