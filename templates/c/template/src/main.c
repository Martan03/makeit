#include <stdio.h>

int main({{ args ? "int argc, char **argv" : "void" }}) {
    printf("Hello World\n");
}
