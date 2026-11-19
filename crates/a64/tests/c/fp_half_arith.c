#include <stdint.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 volatile uint16_t a[]={0x3c00,0x4000,0xc100,0x7c00,0x7d01,0x8000,1,0x7bff},b[]={0x3800,0x4200,0x3400,0,0x7e23,0,2,0x1000};
 for(int i=0;i<8;i++) {
  uint16_t aa=a[i],bb=b[i],result[5];__fp16 x,y,r[5];memcpy(&x,&aa,2);memcpy(&y,&bb,2);
  __asm__("fadd %h0,%h5,%h6\n\tfsub %h1,%h5,%h6\n\tfmul %h2,%h5,%h6\n\tfdiv %h3,%h5,%h6\n\tfsqrt %h4,%h5":"=&w"(r[0]),"=&w"(r[1]),"=&w"(r[2]),"=&w"(r[3]),"=&w"(r[4]):"w"(x),"w"(y));
  memcpy(result,r,sizeof result);for(int j=0;j<5;j++)printf("%04x%c",result[j],j==4?'\n':' ');
 }
}
