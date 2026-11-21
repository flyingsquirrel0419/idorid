#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint8x16_t x) { uint8_t b[16]; vst1q_u8(b,x); for (int i=0;i<16;i++) printf("%02x",b[i]); puts(""); }
int main(void) {
    volatile uint8_t a[16]={0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15};
    volatile uint8_t b[16]={0xfe,0xdc,0xba,0x98,0x76,0x54,0x32,0x10,0xf0,0x0f,0x55,0xaa,1,3,5,7};
    uint8x16_t x=vld1q_u8((const uint8_t*)a), y=vld1q_u8((const uint8_t*)b);
    out(vaeseq_u8(x,y)); out(vaesdq_u8(x,y)); out(vaesmcq_u8(x)); out(vaesimcq_u8(x));
    out(vreinterpretq_u8_p128(vmull_p64(vget_lane_p64(vreinterpret_p64_u8(vget_low_u8(x)),0),vget_lane_p64(vreinterpret_p64_u8(vget_low_u8(y)),0))));
    out(vreinterpretq_u8_p16(vmull_p8(vreinterpret_p8_u8(vget_low_u8(x)),vreinterpret_p8_u8(vget_low_u8(y)))));
}
