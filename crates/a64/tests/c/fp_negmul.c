#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void) {volatile uint64_t in[]={0x7ff8000000000123ULL,0x7ff0000000000123ULL,0x7ff0000000000000ULL,0x3ff0000000000000ULL};for(int i=0;i<4;i++){uint64_t x=in[i],o;double a,b=0,r;memcpy(&a,&x,8);__asm__("fnmul %d0,%d1,%d2":"=w"(r):"w"(a),"w"(b));memcpy(&o,&r,8);printf("%016" PRIx64 "\n",o);}}
