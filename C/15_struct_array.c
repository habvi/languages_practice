#include <stdio.h>
#include <string.h>

void line(void);

typedef struct {
    int year;
    int number;
    char name[60];
} student;

void student_info_from_pointer(student *data) {
    printf("%d %d %s\n", data->year, data->number, data->name);
    return;
}

void student_info_multi(student data[], int count) {
    for (int i = 0; i < count; i++) {
        printf("%d %d %s\n", data[i].year, data[i].number, data[i].name);
    }
    return;
}

int main() {
    student data;
    data.year = 3;
    data.number = 15;
    strcpy(data.name, "Alice");

    student *p_data;
    p_data = &data;
    student_info_from_pointer(p_data);
    line();

    // struct array
    student arr[10];
    arr[0].year = 8;
    strcpy(arr[0].name, "Bob");

    student *p_arr;
    p_arr = &arr[0];
    student_info_from_pointer(p_arr);
    // same
    printf("%s %s %s\n", arr[0].name, arr->name, (*arr).name);
    line();

    arr[1].year = 2;
    arr[1].number = 50;
    strcpy(arr[1].name, "Alice");
    student_info_multi(arr, 2);

    return 0;
}

void line(void) {
    printf("-----\n");
}
