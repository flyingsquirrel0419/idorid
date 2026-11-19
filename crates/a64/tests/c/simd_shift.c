#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t x) { uint32_t b[4]; vst1q_u32(b,x); for (int i=0;i<4;i++) printf("%08x%c",b[i],i==3?'\n':' '); }
int main(void) {
    volatile uint32_t a[4]={0x80000000,0xffffffff,0x76543210,0x12345678};
    volatile uint32_t b[4]={0xaabbccdd,0x12345678,0xff00ff00,0x00ff00ff};
    uint32x4_t x=vld1q_u32((const uint32_t*)a),y=vld1q_u32((const uint32_t*)b);
    out(vshrq_n_u32(x,5)); out(vreinterpretq_u32_s32(vshrq_n_s32(vreinterpretq_s32_u32(x),5)));
    out(vrshrq_n_u32(x,5)); out(vreinterpretq_u32_s32(vrshrq_n_s32(vreinterpretq_s32_u32(x),5)));
    out(vsraq_n_u32(y,x,5)); out(vrsraq_n_u32(y,x,5)); out(vshlq_n_u32(x,5)); out(vsliq_n_u32(y,x,5)); out(vsriq_n_u32(y,x,5));
}
