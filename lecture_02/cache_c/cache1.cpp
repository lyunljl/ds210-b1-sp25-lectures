#include <time.h>
#include <stdio.h>
#include <stdlib.h>

/*
 * Contiguous access

 * Example from https://mecha-mind.medium.com/demystifying-cpu-caches-with-examples-810534628d71
 *
 * compile with `clang cache.cpp -o cache`
 * run with `./cache`
 */

int main(int argc, char* argv[]) {
    const int length = 512 * 1024 * 1024;
    const int cache_line_size = 16;  // size in terms of ints (4 bytes) so 16 * 4 = 64 bytes
    const int m = length/cache_line_size;

    printf("Looping %d M times\n", m/(1024*1024));

    int *arr = (int*)malloc(length * sizeof(int));

    clock_t start = clock();
    for (int i = 0; i < m; i++)   // contiguous access
        arr[i]++;
    clock_t stop = clock();
    
    double duration = ((double)(stop - start)) / CLOCKS_PER_SEC * 1000;
    
    printf("Duration: %f ms\n", duration);

    free(arr);
    return 0;
}

