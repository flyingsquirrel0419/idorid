#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void run(const int8_t *a, const int8_t *b) {
    int8_t out[16]; uint64_t flags;
    int8x16_t x=vld1q_s8(a),y=vld1q_s8(b);
    __asm__ volatile("msr fpsr, xzr" ::: "memory");
    int8x16_t z=vqaddq_s8(x,y);
    __asm__ volatile("mrs %0, fpsr" : "=r"(flags) :: "memory");
    vst1q_s8(out,z);
    for (int i=0;i<16;i++) printf("%02x",(uint8_t)out[i]);
    printf(" %08x\n",(unsigned)flags);
}
int main(void) {
    volatile int8_t x[16]={127,-128,12,-12,10,-10,1,-1,64,-64,11,20,30,40,50,60};
    volatile int8_t y[16]={1,-1,10,-10,3,-3,0,0,64,-65,1,2,3,4,5,6};
    run((const int8_t*)x,(const int8_t*)y);
}
