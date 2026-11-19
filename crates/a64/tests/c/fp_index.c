#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(float32x4_t x) {uint32_t a[4];vst1q_u32(a,vreinterpretq_u32_f32(x));for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' ');}
int main(void) {
 volatile uint32_t a[4]={0x3f800000,0xbf000000,0x7fc00001,0x80000000},b[4]={0x40000000,0x3e800000,0x3f800000,0x3fc00000};
 float32x4_t x=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)a)),y=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)b));
 out(vmulq_laneq_f32(x,y,3));out(vfmaq_laneq_f32(x,x,y,3));out(vfmsq_laneq_f32(x,x,y,3));out(vmulxq_laneq_f32(x,y,3));
}
