#include <arm_neon.h>
#include <stdint.h>
#include <stdio.h>
static void out(uint32x4_t v) { uint32_t a[4];vst1q_u32(a,v);for(int i=0;i<4;i++)printf("%08x%c",a[i],i==3?'\n':' '); }
int main(void) {
 volatile uint32_t a[4]={0x01234567,0x89abcdef,0xfedcba98,0x76543210},b[4]={0x10203040,0x50607080,0x90a0b0c0,0xd0e0f001},c[4]={0xdeadbeef,0xbeeff00d,0xabcdef01,0x87654321};
 uint32x4_t x=vld1q_u32((const uint32_t*)a),y=vld1q_u32((const uint32_t*)b),z=vld1q_u32((const uint32_t*)c);
 out(vsha1cq_u32(x,0x12345678,z));out(vsha1pq_u32(x,0x12345678,z));out(vsha1mq_u32(x,0x12345678,z));
 printf("%08x\n",vsha1h_u32(a[0]));out(vsha1su0q_u32(x,y,z));out(vsha1su1q_u32(x,y));
 out(vsha256hq_u32(x,y,z));out(vsha256h2q_u32(x,y,z));out(vsha256su0q_u32(x,y));out(vsha256su1q_u32(x,y,z));
}
