#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint8x8_t x) {uint8_t a[8];vst1_u8(a,x);for(int i=0;i<8;i++)printf("%02x",a[i]);puts("");}
int main(void) {
 volatile uint16_t a[8]={0,1,511,2047,32767,32768,65535,65000};
 uint16x8_t x=vld1q_u16((const uint16_t*)a);int16x8_t sx=vreinterpretq_s16_u16(x);
 out(vqshrn_n_u16(x,3));out(vreinterpret_u8_s8(vqshrn_n_s16(sx,3)));out(vqshrun_n_s16(sx,3));
 out(vqrshrn_n_u16(x,3));out(vreinterpret_u8_s8(vqrshrn_n_s16(sx,3)));out(vqrshrun_n_s16(sx,3));
}
