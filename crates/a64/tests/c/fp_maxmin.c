#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    volatile uint64_t cases[][2]={{0,0x8000000000000000ULL},{0x3ff0000000000000ULL,0xc000000000000000ULL},{0x7ff8000000000123ULL,0x3ff0000000000000ULL},{0x7ff0000000000123ULL,0x4000000000000000ULL},{0x7ff8000000000123ULL,0x7ff8000000000456ULL}};
    for(unsigned i=0;i<sizeof(cases)/sizeof(cases[0]);i++) {
        uint64_t aa=cases[i][0],bb=cases[i][1],out[4]; double a,b,r[4]; memcpy(&a,&aa,8); memcpy(&b,&bb,8);
        __asm__("fmax %d0,%d4,%d5\n\tfmin %d1,%d4,%d5\n\tfmaxnm %d2,%d4,%d5\n\tfminnm %d3,%d4,%d5" : "=&w"(r[0]),"=&w"(r[1]),"=&w"(r[2]),"=&w"(r[3]) : "w"(a),"w"(b));
        memcpy(out,r,sizeof(out)); for(int j=0;j<4;j++) printf("%016" PRIx64 "%c",out[j],j==3?'\n':' ');
    }
}
