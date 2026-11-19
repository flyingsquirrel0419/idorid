#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
static void out(float16x8_t x) {uint16_t a[8];vst1q_u16(a,vreinterpretq_u16_f16(x));for(int i=0;i<8;i++)printf("%04x%c",a[i],i==7?'\n':' ');}
int main(void) {
 volatile uint16_t a[8]={0x3c00,0x4000,0xc100,0x7c00,0x7d01,0x8000,1,0x7bff},b[8]={0x3800,0x4200,0x3400,0,0x7e23,0,2,0x1000};
 float16x8_t x=vreinterpretq_f16_u16(vld1q_u16((const uint16_t*)a)),y=vreinterpretq_f16_u16(vld1q_u16((const uint16_t*)b));
 out(vmulq_laneq_f16(x,y,3));out(vfmaq_laneq_f16(x,x,y,3));out(vfmsq_laneq_f16(x,x,y,3));out(vmulxq_laneq_f16(x,y,3));
 out(vaddq_f16(x,y));out(vsubq_f16(x,y));out(vmulq_f16(x,y));out(vdivq_f16(x,y));out(vsqrtq_f16(x));out(vfmaq_f16(x,x,y));out(vfmsq_f16(x,x,y));out(vmaxq_f16(x,y));out(vminq_f16(x,y));
}
