#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(float32x4_t f) { uint32_t a[4];vst1q_u32(a,vreinterpretq_u32_f32(f));for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' '); }
int main(void) {
 volatile uint32_t a[4]={0x3f800000,0xbf000000,0x7fc00001,0x80000000},b[4]={0x40000000,0x3e800000,0x3f800000,0};
 float32x4_t x=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)a)),y=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)b));
 out(vabsq_f32(x));out(vnegq_f32(x));out(vsqrtq_f32(x));out(vrndnq_f32(x));out(vrndpq_f32(x));out(vrndmq_f32(x));out(vrndq_f32(x));
 out(vaddq_f32(x,y));out(vsubq_f32(x,y));out(vmulq_f32(x,y));out(vdivq_f32(x,y));
 out(vmaxq_f32(x,y));out(vminq_f32(x,y));out(vmaxnmq_f32(x,y));out(vminnmq_f32(x,y));
 out(vpaddq_f32(x,y));out(vpmaxq_f32(x,y));out(vpminnmq_f32(x,y));
 out(vdupq_n_f32(vmaxvq_f32(y)));out(vdupq_n_f32(vminnmvq_f32(x)));
 out(vreinterpretq_f32_u32(vceqzq_f32(x)));out(vreinterpretq_f32_u32(vcgtzq_f32(x)));out(vreinterpretq_f32_u32(vclezq_f32(x)));out(vreinterpretq_f32_u32(vcageq_f32(x,y)));out(vabdq_f32(x,y));
 out(vfmaq_f32(x,x,y));out(vfmsq_f32(x,x,y));
 out(vreinterpretq_f32_u32(vceqq_f32(x,y)));out(vreinterpretq_f32_u32(vcgtq_f32(x,y)));out(vreinterpretq_f32_u32(vcgeq_f32(x,y)));
}
