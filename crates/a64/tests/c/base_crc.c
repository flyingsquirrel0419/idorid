#include <arm_acle.h>
typedef unsigned long u64;static char out[300];static unsigned pos;
static void hex(u64 x){for(int i=60;i>=0;i-=4)out[pos++]="0123456789abcdef"[(x>>i)&15];out[pos++]='\n';}
void _start(void){unsigned v=0x12345678;u64 x=0xdeadbeef01234567UL;hex(__crc32b(v,x));hex(__crc32h(v,x));hex(__crc32w(v,x));hex(__crc32d(v,x));hex(__crc32cb(v,x));hex(__crc32ch(v,x));hex(__crc32cw(v,x));hex(__crc32cd(v,x));register u64 a __asm__("x0")=1,b __asm__("x1")=(u64)out,c __asm__("x2")=pos,d __asm__("x8")=64;__asm__ volatile("svc 0":"+r"(a):"r"(b),"r"(c),"r"(d):"memory");a=0;d=93;__asm__ volatile("svc 0"::"r"(a),"r"(d):"memory");__builtin_unreachable();}
