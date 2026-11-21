#include <inttypes.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
int main(void) {
    volatile uint64_t values[]={0x3ff0000000000000ULL,0x4000000000000000ULL,0x7ff8000000000101ULL};
    for(unsigned i=0;i<3;i++) for(unsigned j=0;j<3;j++) {
        uint64_t aa=values[i],bb=values[j],flags,out; double a,b,r;memcpy(&a,&aa,8);memcpy(&b,&bb,8);
        __asm__("fcmp %d2,%d3\n\tfcsel %d0,%d2,%d3,lt\n\tfccmp %d2,%d3,#10,ne\n\tmrs %1,nzcv":"=&w"(r),"=r"(flags):"w"(a),"w"(b):"cc");
        memcpy(&out,&r,8);printf("%016" PRIx64 " %08" PRIx64 "\n",out,flags);
    }
}
