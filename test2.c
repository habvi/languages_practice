#include <stdio.h>

int main() {
    int num;
    printf("Enter the integer : ");
    // &はadress参照。integerやcharはいる,stringはいらない。
    scanf("%d", &num);
    printf("The number is .. %d\n", num);


    char name[20];
    printf("Enter your name : ");

    // 空白まで
    scanf("%s", name);
    printf("Your name is .. %s\n", name);

    // 指定文字数まで空白があっても1行
    fgets(name, 20, stdin);
    printf("Your name is .. %s\n", name);

    return 0;
}