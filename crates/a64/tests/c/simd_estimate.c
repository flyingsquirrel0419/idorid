#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
int main(void) {
 uint64_t hash=1469598103934665603ULL;
 for (unsigned i=0;i<1024;i+=4) {
    uint32_t a[4];for(int j=0;j<4;j++)a[j]=(i+j)*0x400000u+0x12345u;
    uint32x4_t x=vld1q_u32(a);uint32_t r[4],s[4];
    vst1q_u32(r,vrecpeq_u32(x));vst1q_u32(s,vrsqrteq_u32(x));
    for(int j=0;j<4;j++){hash=(hash^r[j])*1099511628211ULL;hash=(hash^s[j])*1099511628211ULL;}
 }
 printf("%016llx\n",(unsigned long long)hash);
}
