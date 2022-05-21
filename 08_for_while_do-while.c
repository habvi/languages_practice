#include <stdio.h>
#include <string.h>

int main() {
    int n = 11;
    for (int i = 0; i < n; i++) {
        printf("%03d\n", i);
    }


    int m = 1;
    while (m <= 1000) {
        printf("%d\n", m);
        m *= 2;
    }


    // executed at least once
    // effective when checking input
    int r;
    double s;

    do {
        printf("円の半径は？ : ");
        scanf("%d", &r);
    } while (r < 0);

    s = r * r * 3.14;
    printf("円の面積: %f\n", s);

    return 0;
}
