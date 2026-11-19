typedef unsigned long u64;
static char out[1024];static unsigned pos;
static void hex(u64 x){for(int i=60;i>=0;i-=4)out[pos++]="0123456789abcdef"[(x>>i)&15];out[pos++]='\n';}
static u64 values[2] __attribute__((aligned(16)))={123,456};
void _start(void){
 u64 x,y,status,flags;u64 *p=values;
 __asm__ volatile("ldxp %0,%1,[%2]":"=&r"(x),"=&r"(y):"r"(p):"memory");hex(x);hex(y);
 __asm__ volatile("stxp %w0,%2,%3,[%1]":"=&r"(status):"r"(p),"r"(789UL),"r"(1234UL):"memory");hex(status);hex(values[0]);hex(values[1]);
 register u64 a __asm__("x0")=789,b __asm__("x1")=1234,c __asm__("x2")=55,d __asm__("x3")=66;
 __asm__ volatile("caspal x0,x1,x2,x3,[%4]":"+r"(a),"+r"(b),"+r"(c),"+r"(d):"r"(p):"memory");hex(a);hex(b);hex(values[0]);hex(values[1]);
 __asm__ volatile("msr nzcv,%1; setf8 %w2; mrs %0,nzcv":"=r"(flags):"r"(0x20000000UL),"r"(0x180UL):"cc");hex(flags);
 __asm__ volatile("msr nzcv,%1; rmif %2,#17,#9; mrs %0,nzcv":"=r"(flags):"r"(0x60000000UL),"r"(0x120000UL):"cc");hex(flags);
 __asm__ volatile("msr nzcv,%1; axflag; mrs %0,nzcv":"=r"(flags):"r"(0x30000000UL):"cc");hex(flags);
 __asm__ volatile("msr nzcv,%1; xaflag; mrs %0,nzcv":"=r"(flags):"r"(0x40000000UL):"cc");hex(flags);
 unsigned int pair[2] __attribute__((aligned(8)))={100,200};
 register unsigned int w0 __asm__("w0")=100,w1 __asm__("w1")=200,w2 __asm__("w2")=11,w3 __asm__("w3")=22;
 __asm__ volatile("casp w0,w1,w2,w3,[%4]":"+r"(w0),"+r"(w1),"+r"(w2),"+r"(w3):"r"(pair):"memory");hex(w0);hex(w1);hex(pair[0]);hex(pair[1]);
 register u64 x0 __asm__("x0")=1,x1 __asm__("x1")=(u64)out,x2 __asm__("x2")=pos,x8 __asm__("x8")=64;
 __asm__ volatile("svc 0":"+r"(x0):"r"(x1),"r"(x2),"r"(x8):"memory");x0=0;x8=93;__asm__ volatile("svc 0"::"r"(x0),"r"(x8):"memory");__builtin_unreachable();
}
