#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 uint64_t old;__asm__ volatile("mrs %0,fpcr":"=r"(old));uint64_t f=1ULL<<19;__asm__ volatile("msr fpcr,%0"::"r"(f));
 volatile uint16_t a[]={1,0x8001,0x0400,0x0400,0x7c00},b[]={0x3c00,0x3c00,0x3800,0x0400,1};
 for(int i=0;i<5;i++) {
  uint16_t aa=a[i],bb=b[i],out;__fp16 x,y,r;uint64_t flags;memcpy(&x,&aa,2);memcpy(&y,&bb,2);
  __asm__ volatile("msr fpsr,xzr\n\tfmul %h0,%h2,%h3\n\tmrs %1,fpsr":"=&w"(r),"=r"(flags):"w"(x),"w"(y):"memory");memcpy(&out,&r,2);printf("%04x %02llx\n",out,(unsigned long long)flags);
 }
 __asm__ volatile("msr fpcr,%0"::"r"(old));
}
