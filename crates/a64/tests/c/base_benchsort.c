#include <stdio.h>
#include <stdlib.h>
static int cmp(const void *p,const void *q){int a=*(const int *)p,b=*(const int *)q;return (a>b)-(a<b);}
int main(void){int a[512];unsigned x=1,sum=0;for(int n=0;n<1500;n++){for(int i=0;i<512;i++){x=x*1664525u+1013904223u;a[i]=(int)x;}qsort(a,512,sizeof a[0],cmp);sum+=(unsigned)a[n&511];}printf("%u\n",sum);}
