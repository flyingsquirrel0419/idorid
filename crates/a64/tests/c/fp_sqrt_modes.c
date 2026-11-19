#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void){uint64_t old;__asm__ volatile("mrs %0,fpcr":"=r"(old));volatile uint64_t values[]={0x4000000000000000ULL,0x3ff0000000000000ULL,0x0010000000000000ULL,1ULL,0x7fefffffffffffffULL,0xbff0000000000000ULL,0x8000000000000000ULL};for(int mode=0;mode<4;mode++){uint64_t fpcr=(uint64_t)mode<<22;__asm__ volatile("msr fpcr,%0"::"r"(fpcr):"memory");for(unsigned i=0;i<sizeof(values)/sizeof(values[0]);i++){uint64_t bits=values[i],out,flags;double x,r;memcpy(&x,&bits,8);__asm__ volatile("msr fpsr,xzr\n\tfsqrt %d0,%d2\n\tmrs %1,fpsr":"=&w"(r),"=r"(flags):"w"(x):"memory");memcpy(&out,&r,8);printf("%016" PRIx64 " %02" PRIx64 "\n",out,flags);}}__asm__ volatile("msr fpcr,%0"::"r"(old));}
