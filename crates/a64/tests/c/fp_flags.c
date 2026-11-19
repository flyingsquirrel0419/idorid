#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 volatile uint64_t a[]={0x3ff0000000000000ULL,0x3ff0000000000000ULL,0x0010000000000000ULL,0x7fefffffffffffffULL};
 volatile uint64_t b[]={0x3ca0000000000000ULL,0x4008000000000000ULL,0x3fd5555555555555ULL,0x4000000000000000ULL};
 for(int i=0;i<4;i++) {
   uint64_t aa=a[i],bb=b[i],flags,out;double x,y,r;memcpy(&x,&aa,8);memcpy(&y,&bb,8);
   __asm__ volatile("msr fpsr,xzr":::"memory");
   if(i==0)__asm__ volatile("fadd %d0,%d1,%d2":"=w"(r):"w"(x),"w"(y));
   else if(i==1)__asm__ volatile("fdiv %d0,%d1,%d2":"=w"(r):"w"(x),"w"(y));
   else __asm__ volatile("fmul %d0,%d1,%d2":"=w"(r):"w"(x),"w"(y));
   __asm__ volatile("mrs %0,fpsr":"=r"(flags)::"memory");memcpy(&out,&r,8);printf("%016" PRIx64 " %02" PRIx64 "\n",out,flags);
 }
}
