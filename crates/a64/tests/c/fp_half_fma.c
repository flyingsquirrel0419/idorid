#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 uint64_t hash=1469598103934665603ULL;uint32_t state=42;
 for(int i=0;i<30000;i++) {
   uint16_t raw[3];for(int j=0;j<3;j++){state=state*1664525u+1013904223u;raw[j]=(uint16_t)(state>>16);}
   __fp16 x,y,z,r;uint16_t out;memcpy(&x,&raw[0],2);memcpy(&y,&raw[1],2);memcpy(&z,&raw[2],2);
   __asm__("fmadd %h0,%h1,%h2,%h3":"=w"(r):"w"(x),"w"(y),"w"(z));memcpy(&out,&r,2);hash=(hash^out)*1099511628211ULL;
 }
 printf("%016llx\n",(unsigned long long)hash);
}
