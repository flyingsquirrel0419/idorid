#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint64x2_t x) {uint64_t a[2];vst1q_u64(a,x);printf("%016llx %016llx\n",(unsigned long long)a[0],(unsigned long long)a[1]);}
int main(void) {
 volatile uint64_t data[2]={0x3ff0000010000000ULL,0xc000000000000001ULL};
 volatile uint32_t inp[4]={0x3f800000,0x7fc00123,0x33800000,0xc0400000};
 float64x2_t x=vreinterpretq_f64_u64(vld1q_u64((const uint64_t*)data));float32x4_t y=vreinterpretq_f32_u32(vld1q_u32((const uint32_t*)inp));
 out(vreinterpretq_u64_f32(vcombine_f32(vcvt_f32_f64(x),vcvtx_f32_f64(x))));
 out(vreinterpretq_u64_f64(vcvt_f64_f32(vget_high_f32(y))));
 out(vreinterpretq_u64_f16(vcombine_f16(vcvt_f16_f32(y),vcvt_f16_f32(y))));
 out(vreinterpretq_u64_f32(vcvt_f32_f16(vcvt_f16_f32(y))));
}
