typedef unsigned long u64;static char out[1024];static unsigned pos;static void hex(u64 x){for(int i=60;i>=0;i-=4)out[pos++]="0123456789abcdef"[(x>>i)&15];out[pos++]='\n';}
static unsigned char data[64] __attribute__((aligned(16)))={0x80,0x81,0x82,0x83,0x84,0x85,0x86,0x87};
void _start(void){u64 a,b,c;unsigned char *p=data;
__asm__ volatile("ldaprb %w0,[%1]":"=r"(a):"r"(p):"memory");hex(a);
__asm__ volatile("ldapursb %0,[%1,#1]":"=r"(a):"r"(p):"memory");hex(a);
__asm__ volatile("ldapursh %0,[%1,#2]":"=r"(a):"r"(p):"memory");hex(a);
__asm__ volatile("ldtrsb %0,[%1,#1]":"=r"(a):"r"(p):"memory");hex(a);
__asm__ volatile("ldtrh %w0,[%1,#2]":"=r"(a):"r"(p):"memory");hex(a);
__asm__ volatile("sttrb %w1,[%0,#7]"::"r"(p),"r"(0xaa):"memory");
__asm__ volatile("stlurb %w1,[%0,#6]"::"r"(p),"r"(0xbb):"memory");
__asm__ volatile("ldp %0,%1,[%2]":"=r"(a),"=r"(b):"r"(p):"memory");hex(a);hex(b);
__asm__ volatile("ldrsw %0,[%1,%w2,sxtw #2]":"=r"(a):"r"(p+4),"r"(-1):"memory");hex(a);
__asm__ volatile("ldapur %0,[%1,#0]":"=r"(c):"r"(p):"memory");hex(c);
register u64 x0 __asm__("x0")=1,x1 __asm__("x1")=(u64)out,x2 __asm__("x2")=pos,x8 __asm__("x8")=64;__asm__ volatile("svc 0":"+r"(x0):"r"(x1),"r"(x2),"r"(x8):"memory");x0=0;x8=93;__asm__ volatile("svc 0"::"r"(x0),"r"(x8):"memory");__builtin_unreachable();}
