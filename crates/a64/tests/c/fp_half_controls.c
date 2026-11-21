#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 uint64_t old;__asm__ volatile("mrs %0,fpcr":"=r"(old));uint64_t hash=1469598103934665603ULL;
 volatile uint64_t samples[]={0x3e70000000000000ULL,0x3ff0000000000000ULL,0x40effc0000000000ULL,0x40effe0000000000ULL,0x4100000000000000ULL,0x7ff0000000000000ULL,0xfff8000000001234ULL};
 for(unsigned mode=0;mode<8;mode++) {
   uint64_t f=((uint64_t)(mode&3)<<22)|((uint64_t)(mode>>2)<<26)|(1ULL<<19);
   __asm__ volatile("msr fpcr,%0"::"r"(f):"memory");
   for(unsigned i=0;i<sizeof(samples)/sizeof(samples[0]);i++) {
    uint64_t bits=samples[i];double x,back;__fp16 h;uint16_t hv;uint64_t out,flags;memcpy(&x,&bits,8);
    __asm__ volatile("msr fpsr,xzr\n\tfcvt %h0,%d3\n\tfcvt %d1,%h0\n\tmrs %2,fpsr":"=&w"(h),"=&w"(back),"=r"(flags):"w"(x):"memory");
    memcpy(&hv,&h,2);memcpy(&out,&back,8);hash=(hash^hv)*1099511628211ULL;hash=(hash^out)*1099511628211ULL;hash=(hash^flags)*1099511628211ULL;
   }
 }
 __asm__ volatile("msr fpcr,%0"::"r"(old));printf("%016llx\n",(unsigned long long)hash);
}
