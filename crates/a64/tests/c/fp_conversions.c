#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    volatile uint64_t values[] = {0x3ff8000000000000ULL, 0xc006000000000000ULL, 0x3fd0000000000000ULL, 0x41e0000000000000ULL, 0xc1e0000000200000ULL, 0x7ff0000000000000ULL, 0x7ff8000000000000ULL};
    for (unsigned i=0;i<sizeof(values)/sizeof(values[0]);i++) {
        uint64_t bits=values[i]; double x; memcpy(&x, &bits, sizeof(x)); int64_t n,p,m,z; uint32_t u;
        __asm__("fcvtns %0,%d5\n\tfcvtps %1,%d5\n\tfcvtms %2,%d5\n\tfcvtzs %3,%d5\n\tfcvtzu %w4,%d5" : "=r"(n),"=r"(p),"=r"(m),"=r"(z),"=r"(u) : "w"(x));
        printf("%016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %08" PRIx32 "\n",(uint64_t)n,(uint64_t)p,(uint64_t)m,(uint64_t)z,u);
    }
    return 0;
}
