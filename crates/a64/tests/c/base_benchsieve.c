#include <stdio.h>
#include <string.h>
static unsigned char prime[5001];
int main(void){unsigned sum=0;for(int iteration=0;iteration<3000;iteration++){int max=4000+iteration%1000;memset(prime,1,max+1);for(int p=2;p*p<=max;p++)if(prime[p])for(int n=p*p;n<=max;n+=p)prime[n]=0;for(int p=2;p<=max;p++)if(prime[p])sum+=(unsigned)p;}printf("%u\n",sum);}
