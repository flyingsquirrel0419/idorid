#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t x) {uint32_t a[4];vst1q_u32(a,x);for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' ');}
int main(void) {
 volatile uint32_t a[4]={0x80000000,0x7fffffff,0xfffffff9,12345};volatile int32_t b[4]={1,-32,-1,35};
 uint32x4_t x=vld1q_u32((const uint32_t*)a);int32x4_t y=vld1q_s32((const int32_t*)b), sx=vreinterpretq_s32_u32(x);
 out(vshlq_u32(x,y));out(vreinterpretq_u32_s32(vshlq_s32(sx,y)));out(vrshlq_u32(x,y));out(vreinterpretq_u32_s32(vrshlq_s32(sx,y)));
 out(vqshlq_u32(x,y));out(vreinterpretq_u32_s32(vqshlq_s32(sx,y)));out(vqrshlq_u32(x,y));out(vreinterpretq_u32_s32(vqrshlq_s32(sx,y)));
}
