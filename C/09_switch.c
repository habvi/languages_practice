#include <stdio.h>

int main() {
    char grade = 'A';

    switch(grade) {
    case 'A':
        printf("A!! great!!");
        break;
    case 'B':
        printf("B!! soso!");
        break;
    case 'C':
        printf("C!! maybe bad ..");
        break;
    default:
        printf("Invalid grade..");
    }

    return 0;
}
