#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    volatile uint64_t inputs[] = {0x3fe0000000000000ULL,0xbfe0000000000000ULL,0x4004000000000000ULL,0xc006000000000000ULL,0x7ff0000000000000ULL,0x7ff8000000000123ULL};
    for (unsigned i=0;i<sizeof(inputs)/sizeof(inputs[0]);i++) {
        uint64_t bits=inputs[i], result[5]; double x,a,b,c,d,e; memcpy(&x,&bits,8);
        __asm__("frintn %d0,%d5\n\tfrintp %d1,%d5\n\tfrintm %d2,%d5\n\tfrintz %d3,%d5\n\tfrinta %d4,%d5" : "=w"(a),"=w"(b),"=w"(c),"=w"(d),"=w"(e) : "w"(x));
        memcpy(&result[0],&a,8); memcpy(&result[1],&b,8); memcpy(&result[2],&c,8); memcpy(&result[3],&d,8); memcpy(&result[4],&e,8);
        for (unsigned j=0;j<5;j++) printf("%016" PRIx64 "%c",result[j],j==4?'\n':' ');
    }
}
