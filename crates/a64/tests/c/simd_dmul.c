#include <arm_neon.h>
#include <stdio.h>
#include <stdint.h>
static void out(int16x8_t x) {uint16_t a[8];vst1q_u16(a,vreinterpretq_u16_s16(x));for(int i=0;i<8;i++)printf("%04x%c",a[i],i==7?'\n':' ');}
int main(void) {
 volatile int16_t a[8]={-32768,-32768,32767,-12345,5432,3,-1,1234},b[8]={-32768,1,32767,22222,-32768,-7,-1,5678};
 int16x8_t x=vld1q_s16((const int16_t*)a),y=vld1q_s16((const int16_t*)b);
 out(vqdmulhq_laneq_s16(x,y,3));out(vqrdmulhq_laneq_s16(x,y,2));out(vqrdmlahq_laneq_s16(x,x,y,4));
 out(vqdmulhq_s16(x,y));out(vqrdmulhq_s16(x,y));out(vqrdmlahq_s16(x,x,y));out(vqrdmlshq_s16(x,x,y));
 out(vreinterpretq_s16_s32(vqdmull_s16(vget_low_s16(x),vget_low_s16(y))));out(vreinterpretq_s16_s32(vqdmlal_s16(vdupq_n_s32(123),vget_low_s16(x),vget_low_s16(y))));
}
