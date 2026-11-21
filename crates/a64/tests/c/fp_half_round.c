#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    uint64_t hash=0xcbf29ce484222325ULL, bits=1;
    for (unsigned i=0;i<30000;i++) {
        bits=bits*6364136223846793005ULL+1442695040888963407ULL;
        uint64_t in=(bits & 0x800fffffffffffffULL) | ((uint64_t)(1000+(bits>>48)%55)<<52);
        double x; __fp16 h; uint16_t out; memcpy(&x,&in,8);
        __asm__("fcvt %h0,%d1":"=w"(h):"w"(x)); memcpy(&out,&h,2);
        hash=(hash ^ out)*0x100000001b3ULL;
    }
    printf("%016llx\n",(unsigned long long)hash);
}
