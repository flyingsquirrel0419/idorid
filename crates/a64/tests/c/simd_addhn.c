#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint8x8_t x) {uint8_t a[8];vst1_u8(a,x);for(int i=0;i<8;i++)printf("%02x",a[i]);puts("");}
int main(void) {
 volatile uint16_t a[8]={0,1,127,128,255,32768,65535,50000},b[8]={1,255,128,255,65535,32768,1,12000};
 uint16x8_t x=vld1q_u16((const uint16_t*)a),y=vld1q_u16((const uint16_t*)b);
 out(vaddhn_u16(x,y));out(vraddhn_u16(x,y));out(vsubhn_u16(x,y));out(vrsubhn_u16(x,y));
}
