#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t v) {uint32_t a[4];vst1q_u32(a,v);for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' ');}
int main(void) {
 volatile uint32_t a[4]={0x7fffffff,0x80000000,1234567,0xffffffff},b[4]={0x3fc00000,0xc0360000,0x7f800000,0x7fc01234};
 uint32x4_t x=vld1q_u32((const uint32_t*)a);float32x4_t y=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)b));
 out(vreinterpretq_u32_f32(vcvtq_f32_u32(x)));out(vreinterpretq_u32_f32(vcvtq_f32_s32(vreinterpretq_s32_u32(x))));out(vcvtq_u32_f32(y));out(vreinterpretq_u32_s32(vcvtq_s32_f32(y)));
 out(vreinterpretq_u32_f32(vcvtq_n_f32_u32(x,12)));out(vcvtq_n_u32_f32(y,12));
}
