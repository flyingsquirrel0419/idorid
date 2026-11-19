#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    volatile uint64_t values[]={0x1000001ULL,0x2000001ULL,0x7fffffffffffffffULL,0x8000000000000001ULL,0xffffffffffffffffULL};
    uint64_t original; __asm__ volatile("mrs %0, fpcr":"=r"(original));
    for (unsigned mode=0;mode<4;mode++) {
        uint64_t fpcr=(uint64_t)mode<<22; __asm__ volatile("msr fpcr,%0"::"r"(fpcr));
        for (unsigned i=0;i<5;i++) {
            uint64_t x=values[i]; float a,b; double c; uint32_t ab,bb; uint64_t cb;
            __asm__("scvtf %s0,%3\n\tucvtf %s1,%3\n\tscvtf %d2,%3, #17":"=&w"(a),"=&w"(b),"=&w"(c):"r"(x));
            memcpy(&ab,&a,4);memcpy(&bb,&b,4);memcpy(&cb,&c,8);
            printf("%08" PRIx32 " %08" PRIx32 " %016" PRIx64 "\n",ab,bb,cb);
        }
    }
    __asm__ volatile("msr fpcr,%0"::"r"(original));
}
