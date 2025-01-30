#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * Example of buffer overflow
 *
 * 1. guesses of string length <= 20
 * 2. guesses of string length > 20
 * 3. guesses of string length >> 20
 *
 * compile with `clang unsafe.c -o unsafe`
 * run with `./unsafe`
 */

int main(){
    char loop_bool[20];
    char secretString[20];
    char givenString[20];
    char x;
    int i, ret;

    memset(&loop_bool, 0, 20);
    for (i=0;i<19;i++) {
      x = 'a' + random() % 26; 
      secretString[i] = x;
    }
    printf("secretString: %s\n", secretString);
    while (!loop_bool[0]) { 
        gets(givenString);
        ret = strncmp(secretString, givenString, 20);
        if (0 == ret) {
            printf("SUCCESS!\n");
	    break;
	}else if (ret < 0){
	    printf("LESS!\n");
	} else {
	    printf("MORE!\n");
        }
        printf("secretString: %s\n", secretString);
    }
    printf("secretString: %s\n", secretString);
    printf("givenString: %s\n", givenString);
    return 0;
}