#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
int main(void) {
    volatile uint8_t x[16] = {255,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15};
    volatile uint8_t y[16] = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};
    uint8_t z[16];
    uint8x16_t a = vld1q_u8((const uint8_t *)x), b = vld1q_u8((const uint8_t *)y);
    vst1q_u8(z, vaddq_u8(a,b));
    for (int i=0;i<16;i++) printf("%02x",z[i]);
    puts("");
    return 0;
}
