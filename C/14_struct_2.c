#include <stdio.h>
#include <string.h>

void line(void);

typedef struct {
    int year;
    int number;
    char name[60];
} student;

void student_info(student data) {
    printf("%d %d %s\n", data.year, data.number, data.name);
    return;
}

void change_in_func(student data) {
    // just cooy, original doesn't change
    data.year = 10000;
}

void student_info_from_pointer(student *data) {
    data->year = 7;
    printf("%d %d %s\n", data->year, data->number, data->name);
    return;
}

int main() {
    student data;
    data.year = 3;
    data.number = 15;
    strcpy(data.name, "Alice");
    student_info(data);

    change_in_func(data);
    student_info(data);
    line();

    student *p_data;
    p_data = &data;
    // same
    (*p_data).number = 6;
    student_info(data);
    p_data->number = 35;
    student_info(data);
    line();

    student_info_from_pointer(&data);
    line();

    return 0;
}

void line(void) {
    printf("-----\n");
}
