// https://www.youtube.com/watch?v=zuegQmMdy8M

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

int cash = 100;

void play(int bet) {
    char c[3] = {'J', 'Q', 'K'};
    printf("suffling..\n");
    // seeding random number generator
    srand(time(NULL));

    for (int i = 0; i < 5; i++) {
        int x = rand() % 3;
        int y = rand() % 3;
        int temp = c[x];
        c[x] = c[y];
        c[y] = temp;
    }
    int players_guess;
    printf("what's the position if queen 1, 2, or 3 ?? -> ");
    scanf("%d", &players_guess);
    if (c[players_guess - 1] == 'Q') {
        cash += 3 * bet;
        printf("you win!! result = %c %c %c, total cash = %d\n", c[0], c[1], c[2], cash);
    } else {
        cash -= bet;
        printf("you lose!! result = %c %c %c, total cash = %d\n", c[0], c[1], c[2], cash);
    }
}

void main(void) {
    int bet;
    printf("welcome~. you have %d cash.\n", cash);
    while (cash) {
        printf("what's yout bet?? -> $ ");
        scanf("%d", &bet);
        if (bet == 0 || bet > cash) {
            break;
        }
        play(bet);
    }
}