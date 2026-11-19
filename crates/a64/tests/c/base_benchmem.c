#include <stdio.h>
#include <string.h>
static char a[4096],b[4096];
int main(void){unsigned sum=0;memset(a,'a',sizeof a-1);for(int n=0;n<100000;n++){int len=64+(n%4000);memcpy(b,a,len);b[len]=0;sum+=(unsigned)strlen(b);memmove(a+1,a,len-1);a[0]=(char)('a'+(n&15));}printf("%u\n",sum);}
