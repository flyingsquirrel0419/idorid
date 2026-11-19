#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t x) {uint32_t a[4];vst1q_u32(a,x);for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' ');}
int main(void) {
 volatile uint8_t a[16]={0,1,127,128,255,200,5,7,9,100,111,112,113,114,115,116},b[16]={4,5,6,7,8,9,10,11,12,13,14,15,255,254,253,252};
 uint8x16_t x=vld1q_u8((const uint8_t*)a),y=vld1q_u8((const uint8_t*)b);
 out(vdotq_u32(vdupq_n_u32(123),x,y));out(vreinterpretq_u32_s32(vdotq_s32(vdupq_n_s32(-123),vreinterpretq_s8_u8(x),vreinterpretq_s8_u8(y))));
 out(vdotq_laneq_u32(vdupq_n_u32(12),x,y,3));out(vreinterpretq_u32_s32(vdotq_laneq_s32(vdupq_n_s32(-12),vreinterpretq_s8_u8(x),vreinterpretq_s8_u8(y),2)));
}
