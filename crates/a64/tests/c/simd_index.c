#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint16x8_t x) {uint16_t a[8];vst1q_u16(a,x);for(int i=0;i<8;i++)printf("%04x%c",a[i],i==7?'\n':' ');}
int main(void) {
 volatile uint16_t a[8]={0,1,32767,32768,65535,23,100,500},b[8]={7,8,9,1000,111,222,333,444};
 uint16x8_t x=vld1q_u16((const uint16_t*)a), y=vld1q_u16((const uint16_t*)b);
 out(vmulq_laneq_u16(x,y,3)); out(vmlaq_laneq_u16(x,y,y,6));
 out(vreinterpretq_u16_u32(vmull_laneq_u16(vget_low_u16(x),y,5)));
 out(vreinterpretq_u16_u32(vmlal_laneq_u16(vdupq_n_u32(123),vget_low_u16(x),y,4)));
}
