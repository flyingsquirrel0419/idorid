typedef unsigned long u64;
static char out[4096]; static unsigned pos;
static void hex(u64 n) { for(int i=60;i>=0;i-=4) out[pos++]="0123456789abcdef"[(n>>i)&15];out[pos++]='\n'; }
__attribute__((noinline)) static u64 f(u64 a,u64 b) { return ((a+b) ^ (a-b)) + (a*b) + (b ? a/b : 0) + (a >> (b&63)); }
void _start(void) {
 for(u64 i=0;i<32;i++) {u64 a=0x9123456789abcdefUL^(i*0x123456789abcdefUL);u64 b=i*0xabcdef1234UL+1;hex(f(a,b));hex(__builtin_bswap64(a));hex(__builtin_clzll(a));}
 register u64 x0 __asm__("x0")=1, x1 __asm__("x1")=(u64)out, x2 __asm__("x2")=pos, x8 __asm__("x8")=64;
 __asm__ volatile("svc 0" : "+r"(x0) : "r"(x1),"r"(x2),"r"(x8) : "memory");
 x0=0;x8=93;__asm__ volatile("svc 0" :: "r"(x0),"r"(x8) : "memory");__builtin_unreachable();
}
