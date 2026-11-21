#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
int main(void) {
  uint64_t us[] = {0, 1, 3, 0x8000000000000000ull, 0xffffffffffffffffull, 123456789012345ull};
  int64_t ss[] = {0, -1, 7, INT64_MIN, INT64_MAX, -987654321098ll};
  for (int i = 0; i < 6; i++) {
    printf("%a %a %a %a\n", vcvtd_f64_u64(us[i]), vcvtd_f64_s64(ss[i]), (double)vcvts_f32_u32((uint32_t)us[i]), (double)vcvts_f32_s32((int32_t)ss[i]));
  }
  int32x4_t v = {1, -2, 0x7fffffff, -2147483647 - 1};
  float32x4_t f = vcvtq_f32_s32(v); uint32x4_t uv = {1, 2, 0xffffffffu, 0x80000000u}; float32x4_t g = vcvtq_f32_u32(uv);
  int64x2_t w = {-5, 1ll << 62}; float64x2_t h = vcvtq_f64_s64(w);
  printf("%a %a %a %a | %a %a %a %a | %a %a\n", f[0], f[1], f[2], f[3], g[0], g[1], g[2], g[3], h[0], h[1]);
  return 0;
}
