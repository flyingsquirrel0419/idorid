#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
int main(void) {
    volatile uint64_t src[][3]={{0x3ff8000000000000ULL,0x4004000000000000ULL,0xc008000000000000ULL},{0x7ff8000000000001ULL,0x7ff8000000000002ULL,0x7ff8000000000003ULL},{0x7ff0000000000001ULL,0x7ff0000000000002ULL,0x7ff0000000000003ULL},{0x0000000000000000ULL,0x7ff0000000000000ULL,0x7ff8000000000123ULL}};
    for(int i=0;i<4;i++){
        uint64_t bits[3]={src[i][0],src[i][1],src[i][2]}, out[4]; double a,b,c,r[4];
        memcpy(&a,&bits[0],8); memcpy(&b,&bits[1],8); memcpy(&c,&bits[2],8);
        __asm__("fmadd %d0,%d4,%d5,%d6\n\tfmsub %d1,%d4,%d5,%d6\n\tfnmadd %d2,%d4,%d5,%d6\n\tfnmsub %d3,%d4,%d5,%d6":"=&w"(r[0]),"=&w"(r[1]),"=&w"(r[2]),"=&w"(r[3]):"w"(a),"w"(b),"w"(c));
        memcpy(out,r,sizeof out); for(int j=0;j<4;j++) printf("%016" PRIx64 "%c",out[j],j==3?'\n':' ');
    }
}
