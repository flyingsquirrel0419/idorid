#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
int main(void) {
 uint64_t hash=1469598103934665603ULL;
 for(unsigned exp=0;exp<256;exp++) for(unsigned top=0;top<256;top+=4) {
    uint32_t values[4];for(unsigned j=0;j<4;j++) values[j]=(exp<<23)|((top+j)<<15)|0x1234u;
    float32x4_t x=vreinterpretq_f32_u32(vld1q_u32(values));uint32_t r[4],s[4];
    vst1q_u32(r,vreinterpretq_u32_f32(vrecpeq_f32(x)));vst1q_u32(s,vreinterpretq_u32_f32(vrsqrteq_f32(x)));
    for(unsigned j=0;j<4;j++){hash=(hash^r[j])*1099511628211ULL;hash=(hash^s[j])*1099511628211ULL;}
 }
 printf("%016llx\n",(unsigned long long)hash);
}
