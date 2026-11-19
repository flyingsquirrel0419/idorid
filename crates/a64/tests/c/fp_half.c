#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
    uint64_t hash=0xcbf29ce484222325ULL;
    for (unsigned i=0;i<65536;i++) {
        uint16_t bits=(uint16_t)i, output; __fp16 h,back; float s; uint32_t sbits;
        memcpy(&h,&bits,2);
        __asm__("fcvt %s0,%h2\n\tfcvt %h1,%s0":"=&w"(s),"=&w"(back):"w"(h));
        memcpy(&sbits,&s,4); memcpy(&output,&back,2);
        hash=(hash ^ sbits)*0x100000001b3ULL;
        hash=(hash ^ output)*0x100000001b3ULL;
    }
    printf("%016llx\n",(unsigned long long)hash);
}
