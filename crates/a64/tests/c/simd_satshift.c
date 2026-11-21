#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t x) {uint32_t a[4];vst1q_u32(a,x);for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' ');}
int main(void) {
 volatile uint32_t a[4]={0x80000000,0x7fffffff,0xfffffff9,12345};
 uint32x4_t x=vld1q_u32((const uint32_t*)a);int32x4_t sx=vreinterpretq_s32_u32(x);
 out(vqshlq_n_u32(x,5));out(vreinterpretq_u32_s32(vqshlq_n_s32(sx,5)));out(vqshluq_n_s32(sx,5));
}
