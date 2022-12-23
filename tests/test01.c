// Example to illustrate kilo-rs capabilities
#include <stdio.h>

int integer;

/* Multiline comment in a single line */

float f;

/*
 * Multiline comment spanning multiple lines
 *
 */

struct test {
    int m;
};

/*
 * // single line comments inside multiline comments
 * // are not special
 *
 */

typedef struct mystruct {
    int n;
} mytype_s;

/* multiline comment with no separator */ int m = 20;
/* multiline comment with a separator  */int n = 20;
int p = 10; /* multiline comment enclosing a // comment */int q = 20;

/* I'm a comment */ int r = 10; // I'm too

// This comment has a control character  (Ctrl - F)
// Few more control characters:  

int main() {
    int a = 100;
    int b = 100;

    char *str = "This string /* has no */ comments";
    int sum = a + b;
    printf("sum of %d and %d is %d\n", a, b, sum);
}

