#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 uint64_t hash=1469598103934665603ULL, original; __asm__ volatile("mrs %0, fpcr":"=r"(original));
 for(unsigned mode=0;mode<8;mode++) {
   uint64_t fpcr=(uint64_t)(mode&3)<<22 | (uint64_t)(mode>>2)<<24;
   __asm__ volatile("msr fpcr,%0\n\tmsr fpsr,xzr"::"r"(fpcr):"memory");
   for(unsigned i=0;i<10000;i++) {
      uint64_t bits=((uint64_t)(i*37%2048)<<52)|((uint64_t)(i*61%256)<<44)|(uint64_t)(i*101); if(i&1)bits|=1ULL<<63;
      double x,a,b;uint64_t ra,rb;memcpy(&x,&bits,8);
      __asm__("frecpe %d0,%d2\n\tfrsqrte %d1,%d2":"=&w"(a),"=&w"(b):"w"(x));
      memcpy(&ra,&a,8);memcpy(&rb,&b,8);hash=(hash^ra)*1099511628211ULL;hash=(hash^rb)*1099511628211ULL;
   }
   uint64_t flags;__asm__ volatile("mrs %0,fpsr":"=r"(flags));hash=(hash^flags)*1099511628211ULL;
 }
 __asm__ volatile("msr fpcr,%0"::"r"(original));printf("%016llx\n",(unsigned long long)hash);
}
