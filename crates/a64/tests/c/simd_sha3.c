#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
static void out(uint8x16_t x) {uint8_t a[16];vst1q_u8(a,x);for(int i=0;i<16;i++)printf("%02x",a[i]);puts("");}
int main(void) {
 volatile uint8_t a[16]={0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15},b[16]={255,1,128,33,22,44,10,12,14,16,19,20,21,22,23,24},c[16]={12,34,56,78,90,1,2,3,4,5,6,7,8,9,10,11};
 uint8x16_t x=vld1q_u8((const uint8_t*)a),y=vld1q_u8((const uint8_t*)b),z=vld1q_u8((const uint8_t*)c);
 out(veor3q_u8(x,y,z));out(vbcaxq_u8(x,y,z));out(vreinterpretq_u8_u64(vrax1q_u64(vreinterpretq_u64_u8(x),vreinterpretq_u64_u8(y))));out(vreinterpretq_u8_u64(vxarq_u64(vreinterpretq_u64_u8(x),vreinterpretq_u64_u8(y),7)));
}
