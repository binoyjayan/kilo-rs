
// Example to illustrate kilo-rs capabilities

/* Multiline comment in a single line */

/*
 * // single line comments inside multiline comments
 * // are not special
 *
 */

/* multiline comment with no separator */ int n = 20;
/* multiline comment with a separator  */int n = 20;
/* multiline comment enclosing a // comment */int n = 20;

/* I'm a comment */ int m = 10; // I'm not

// This comment has a control character  (Ctrl - F)
// Few more control characters:  

int main() {
    int a = 100;
    int b = 100;

    char *str = "This string /* has no */ comments";
    int sum = a + b;
    printf("sum of %d and %d is %d\n", a, b, sum);
}

