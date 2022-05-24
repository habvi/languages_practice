#include <stdio.h>
#include <string.h>

struct Student {
    char name[50];
    char major[50];
    int age;
    double gpa;
};

int main() {
    struct Student s1, s2;
    strcpy( s1.name, "Jim" );
    strcpy( s1.major, "Math" );
    s1.age = 22;
    s1.gpa = 3.2;

    printf("%s\n", s1.name);
    printf("%s\n", s1.major);
    printf("%d\n", s1.age);
    printf("%f\n", s1.gpa);

    // copy all
    s2 = s1;
    printf("%s\n", s2.name);
    printf("%s\n", s2.major);
    printf("%d\n", s2.age);
    printf("%f\n", s2.gpa);

    return 0;
}
