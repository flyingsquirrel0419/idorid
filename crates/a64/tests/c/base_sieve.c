#include <stdint.h>
#include <stdio.h>
#include <string.h>
static uint32_t rot(uint32_t x,unsigned n){return x>>n|x<<(32-n);}
int main(void){unsigned char prime[5001];memset(prime,1,sizeof prime);for(int p=2;p*p<=5000;p++)if(prime[p])for(int n=p*p;n<=5000;n+=p)prime[n]=0;unsigned count=0,sum=0;for(int p=2;p<=5000;p++)if(prime[p]){count++;sum+=p;}
uint32_t s[8]={0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};uint32_t w[64]={0x61626380};w[15]=24;for(int i=16;i<64;i++){uint32_t x=w[i-15],y=w[i-2];w[i]=w[i-16]+(rot(x,7)^rot(x,18)^(x>>3))+w[i-7]+(rot(y,17)^rot(y,19)^(y>>10));}for(int i=0;i<64;i++){uint32_t t=s[7]+(rot(s[4],6)^rot(s[4],11)^rot(s[4],25))+((s[4]&s[5])^(~s[4]&s[6]))+w[i]+(uint32_t)i*0x9e3779b9;uint32_t u=(rot(s[0],2)^rot(s[0],13)^rot(s[0],22))+((s[0]&s[1])^(s[0]&s[2])^(s[1]&s[2]));for(int j=7;j>0;j--)s[j]=s[j-1];s[4]+=t;s[0]=t+u;}printf("%u %u",count,sum);for(int i=0;i<8;i++)printf(" %08x",s[i]);puts("");}
