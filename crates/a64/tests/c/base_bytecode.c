#include <stdint.h>
#include <stdio.h>
int main(void){uint64_t r[8]={1,2,3,4,5,6,7,8};static const unsigned char p[]={0,9,18,27,36,45,54,63,10,19,28,37,46,55};for(unsigned n=0;n<5000000;n++){unsigned op=p[n%sizeof p],x=op&7,y=(op>>3)&7;switch(op%7){case 0:r[x]+=r[y];break;case 1:r[x]^=r[y];break;case 2:r[x]*=0x9e3779b97f4a7c15ULL;break;case 3:r[x]=(r[x]>>7)|(r[x]<<57);break;case 4:r[x]-=r[y];break;case 5:r[x]|=r[y];break;default:r[x]&=r[y];}}printf("%llx\n",(unsigned long long)(r[0]^r[1]^r[2]^r[3]^r[4]^r[5]^r[6]^r[7]));}
