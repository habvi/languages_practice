#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

void from_head_pointer(void);
void from_head_node(void);

int main(void) {
    from_head_pointer();
    printf("-----\n");

    from_head_node();
    return 0;
}

// ---------------------------------
typedef struct node {
    int id;
    char name[16];
    int score;
    struct node *next;
} Node, *Nodep;

// Node head pointer
Nodep hp;

void print_node(Nodep p) {
    if (p == NULL) {
        printf("is empty!\n");
        return;
    }
    for (; p != NULL; p = p->next) {
        printf("%d %s %d\n", p->id, p->name, p->score);
    }
    // same
    // while (p != NULL) {
    //     printf("%d %s %d\n", p->id, p->name, p->score);
    //     p = p->next;
    // }
    printf("----- \n");
}

void del_head(Nodep *hpp) {
    Nodep p = hp;
    if (p = *hpp) {
        *hpp = p->next;
        free(p);
    }
}

void from_head_pointer(void) {
    char names[3][16] = {"abc", "defg", "hijkl"};
    int scores[3] = {50, 80, 90};
    int n = 3;

    Nodep p;
    for (int i = 0; i < n; i++) {
        if (i == 0) {
            hp = (Nodep)malloc(sizeof(Node));
            p = hp;
        } else {
            p->next = (Nodep)malloc(sizeof(Node));
            p = p->next;
        }

        p->id = i + 1;
        strcpy(p->name, names[i]);
        p->score = scores[i];
        p->next = NULL;
    }
    print_node(hp);

    del_head(&hp);
    print_node(hp);

    del_head(&hp);
    print_node(hp);

    del_head(&hp);
    print_node(hp);
}

// ---------------------------------
typedef struct node2 {
    int val;
    struct node2 *next;
} Node2, *Node2p;

// head node
Node2 head;

void print_node2(Node2p p) {
    if (p->next == NULL) {
        printf("is empty!\n");
        return;
    }
    for (p = p->next; p != NULL; p = p->next) {
        printf("%d ", p->val);
    }
    printf("\n");
}

// ???
void del_head2(Node2p hp) {
    Node2p p;
    if (p = hp->next) {
        hp->next = p->next;
        free(p);
    }
}

// from head node
void from_head_node(void) {
    // make [2, 6, 10]
    // head pointer
    head.next = (Node2p)malloc(sizeof(Node2));
    head.next->val = 2;
    head.next->next = (Node2p)malloc(sizeof(Node2));
    head.next->next->val = 6;
    head.next->next->next = (Node2p)malloc(sizeof(Node2));
    head.next->next->next->val = 10;
    head.next->next->next->next = NULL;
    print_node2(&head);

    del_head2(&head);
    print_node2(&head);

    del_head2(&head);
    print_node2(&head);

    del_head2(&head);
    print_node2(&head);
}