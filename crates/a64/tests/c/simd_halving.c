#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
static void out(uint8x16_t x) {uint8_t a[16];vst1q_u8(a,x);for(int i=0;i<16;i++)printf("%02x",a[i]);puts("");}
int main(void) {
 volatile uint8_t a[16]={0,1,127,128,255,200,5,7,9,100,111,112,113,114,115,116},b[16]={4,5,6,7,8,9,10,11,12,13,14,15,255,254,253,252};
 uint8x16_t x=vld1q_u8((const uint8_t*)a),y=vld1q_u8((const uint8_t*)b);int8x16_t sx=vreinterpretq_s8_u8(x),sy=vreinterpretq_s8_u8(y);
 out(vhaddq_u8(x,y));out(vreinterpretq_u8_s8(vhaddq_s8(sx,sy)));out(vrhaddq_u8(x,y));out(vreinterpretq_u8_s8(vrhaddq_s8(sx,sy)));out(vhsubq_u8(x,y));out(vreinterpretq_u8_s8(vhsubq_s8(sx,sy)));
}
