#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void){volatile uint64_t inputs[]={0x3ff8000000000000ULL,0xbfe0000000000000ULL,0x41dfffffffe00000ULL,0x41e0000000000000ULL,0x7ff8000000000123ULL,0x7ff0000000000000ULL};for(int i=0;i<6;i++){uint64_t raw=inputs[i],a,b,c,d,f;double x,r[4];memcpy(&x,&raw,8);__asm__ volatile("msr fpsr,xzr\n\tfrint32x %d0,%d5\n\tfrint32z %d1,%d5\n\tfrint64x %d2,%d5\n\tfrint64z %d3,%d5\n\tmrs %4,fpsr":"=&w"(r[0]),"=&w"(r[1]),"=&w"(r[2]),"=&w"(r[3]),"=r"(f):"w"(x):"memory");memcpy(&a,&r[0],8);memcpy(&b,&r[1],8);memcpy(&c,&r[2],8);memcpy(&d,&r[3],8);printf("%016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %016" PRIx64 " %02" PRIx64 "\n",a,b,c,d,f);}}
