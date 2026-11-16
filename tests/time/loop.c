#include <stdio.h>
#include <stdint.h>

int main(void) {
    int64_t i = 0;
    int64_t acc = 0;
    while (i < 1000000) {
        acc = (acc + i * 3) % 1000007;
        i = i + 1;
    }
    printf("%ld\n", acc);
    return 0;
}