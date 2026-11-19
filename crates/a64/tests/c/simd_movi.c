#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
__attribute__((noinline)) void show(const char *n, uint64x2_t v) { printf("%s %016llx %016llx\n", n, (unsigned long long)v[1], (unsigned long long)v[0]); }
int main(void) {
  show("a", vreinterpretq_u64_u32(vcombine_u32(vdup_n_u32(0x1f000000), vdup_n_u32(0))));
  show("b", vreinterpretq_u64_u32(vdupq_n_u32(0x00ab0000)));
  show("c", vreinterpretq_u64_u32(vdupq_n_u32(0x0000cd00)));
  show("d", vreinterpretq_u64_u32(vdupq_n_u32(0xff00ffff)));
  show("e", vreinterpretq_u64_u16(vdupq_n_u16(0x3400)));
  show("f", vreinterpretq_u64_u32(vdupq_n_u32(0x12ffff)));
  show("g", vreinterpretq_u64_u32(vdupq_n_u32(0x40400000)));
  volatile uint32_t x = 7; uint32x4_t v = vdupq_n_u32(x);
  show("h", vreinterpretq_u64_u32(vorrq_u32(v, vdupq_n_u32(0x5500))));
  show("i", vreinterpretq_u64_u32(vbicq_u32(vdupq_n_u32(x | 0xff00), vdupq_n_u32(0xf00))));
  return 0;
}
