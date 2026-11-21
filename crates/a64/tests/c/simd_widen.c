#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void dump(uint8x16_t v) { uint8_t a[16]; vst1q_u8(a,v); for (int i=0;i<16;i++) printf("%02x",a[i]); puts(""); }
int main(void) {
    uint8_t aa[16] = {0,1,127,128,255,200,5,7,9,100,111,112,113,114,115,116};
    uint8_t bb[16] = {4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19};
    uint8x16_t a=vld1q_u8(aa),b=vld1q_u8(bb);
    dump(vreinterpretq_u8_u16(vpaddlq_u8(a)));
    dump(vpminq_u8(a,b));
    dump(vreinterpretq_u8_u16(vmovl_u8(vget_high_u8(a))));
    dump(vreinterpretq_u8_u16(vmlal_u8(vdupq_n_u16(100),vget_low_u8(a),vget_low_u8(b))));
    dump(vrev64q_u8(a));
    dump(vcombine_u8(vqmovn_u16(vdupq_n_u16(300)),vqmovn_u16(vdupq_n_u16(42))));
    printf("%u %u %u\n",vaddlvq_u8(a),vminvq_u8(a),vmaxvq_u8(a));
    return 0;
}
