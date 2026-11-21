#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    volatile uint64_t inputs[]={0x3ff0000000000000ULL,0x3ff0000010000000ULL,0xc000000030000000ULL,0x7ff8000000123456ULL,0x7ff0000000000123ULL,0x8000000000000000ULL};
    for(unsigned i=0;i<sizeof(inputs)/sizeof(inputs[0]);i++) {
        uint64_t bits=inputs[i], backbits; uint32_t outbits; double x,back; float s; memcpy(&x,&bits,8);
        __asm__("fcvt %s0,%d2\n\tfcvt %d1,%s0":"=&w"(s),"=&w"(back):"w"(x));
        memcpy(&outbits,&s,4); memcpy(&backbits,&back,8); printf("%08" PRIx32 " %016" PRIx64 "\n",outbits,backbits);
    }
}
