#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint8x16_t x) {uint8_t a[16];vst1q_u8(a,x);for(int i=0;i<16;i++)printf("%02x",a[i]);puts("");}
int main(void) {
 volatile uint8_t src[64];for(int i=0;i<64;i++)src[i]=(uint8_t)(i*7+3);
 uint8x16x2_t a=vld2q_u8((const uint8_t*)src);uint8x16x3_t b=vld3q_u8((const uint8_t*)src);uint8x16x4_t c=vld4q_u8((const uint8_t*)src);
 out(a.val[0]);out(a.val[1]);out(b.val[0]);out(b.val[1]);out(b.val[2]);out(c.val[0]);out(c.val[3]);
 uint8_t dst[64];vst4q_u8(dst,c);for(int i=0;i<64;i++)printf("%02x",dst[i]);puts("");
 uint8x16_t d=vld1q_dup_u8((const uint8_t*)src+5);out(d);d=vld1q_lane_u8((const uint8_t*)src+7,d,9);out(d);
}
