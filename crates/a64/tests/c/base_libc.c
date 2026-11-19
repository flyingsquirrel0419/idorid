#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <setjmp.h>
#include <stdatomic.h>
static jmp_buf env;
static int cmp(const void *a,const void *b){int x=*(const int*)a,y=*(const int*)b;return (x>y)-(x<y);}
static unsigned fib(unsigned n){return n<2?n:fib(n-1)+fib(n-2);}
static void jump(void){longjmp(env,7);}
int main(void){
 char a[256],b[256],text[80];unsigned long h=0;for(int len=0;len<100;len++)for(int align=0;align<8;align++){memset(a,0,sizeof a);memset(b,0,sizeof b);memset(a+align,'a'+len%26,len);memcpy(b+align,a+align,len);memmove(b+align+1,b+align,len);h=h*33+strlen(a+align)+memcmp(a+align,b+align+1,len);}
 int arr[97];for(int i=0;i<97;i++)arr[i]=(i*71)%97;qsort(arr,97,sizeof(int),cmp);for(int i=0;i<97;i++)h=h*33+arr[i];
 struct node{struct node *next;int x;} *head=0;for(int i=0;i<200;i++){struct node *n=malloc(sizeof(*n));n->x=i;n->next=head;head=n;}while(head){struct node *n=head;h=h*33+n->x;head=n->next;free(n);}
 _Atomic unsigned long v=5;unsigned long old=atomic_fetch_add(&v,7);unsigned long expected=12;int ok=atomic_compare_exchange_strong(&v,&expected,99);
 int j=setjmp(env);if(!j)jump();snprintf(text,sizeof text,"%lu:%u:%d:%lu:%lu:%d",h,fib(12),j,old,(unsigned long)v,ok);puts(text);return 0;
}
