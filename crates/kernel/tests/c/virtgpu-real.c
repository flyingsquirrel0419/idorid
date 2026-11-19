#include <stdint.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <poll.h>
struct param { uint64_t param, value; };
struct init { uint32_t count, pad; uint64_t params; };
struct blob { uint32_t mem, flags, bo, res; uint64_t size; uint32_t pad, cmd_size; uint64_t cmd, id; };
struct create { uint32_t target,format,bind,width,height,depth,array_size,last_level,nr_samples,flags,bo,res,size,stride; };
struct transfer { uint32_t handle, x,y,z,w,h,d,level,offset,stride,layer_stride; };
struct prime { uint32_t handle, flags; int32_t fd; };
struct exec { uint32_t flags,size; uint64_t command, handles; uint32_t num; int32_t fd; uint32_t ring, stride, n_in, n_out; uint64_t in_sync, out_sync; };
struct map { uint64_t offset; uint32_t handle, pad; };
struct address_alloc { uint64_t size, offset, phys; };
struct address_ping { uint64_t offset,size,metadata; uint32_t version,wait_fd,wait_flags,direction; };
static int address_allocator(void) {
    int fd=open("/dev/goldfish_address_space",O_RDWR); if(fd<0){perror("address open");return 50;}
    struct address_ping p={.metadata=5,.version=sizeof(p)};
    if(ioctl(fd,0xc028470c,&p) || p.metadata!=0){perror("address select");return 51;}
    struct address_alloc a={.size=256};
    if(ioctl(fd,0xc018470a,&a) || a.size!=4096 || a.phys<=a.offset){perror("address alloc");return 52;}
    p.offset=a.offset; p.size=a.size; p.metadata=1;
    if(ioctl(fd,0xc028470c,&p) || p.metadata){perror("address ping");return 53;}
    volatile uint32_t *v=mmap(0,a.size,PROT_READ|PROT_WRITE,MAP_SHARED,fd,a.offset);
    if(v==MAP_FAILED){perror("address mmap");return 54;}
    *v=0xcafebabe; if(*v!=0xcafebabe)return 55;
    int fd2=open("/dev/goldfish_address_space",O_RDWR); if(fd2<0)return 57;
    volatile uint32_t *v2=mmap(0,a.size,PROT_READ|PROT_WRITE,MAP_SHARED,fd2,a.offset);
    if(v2==MAP_FAILED || *v2!=*v)return 58;
    munmap((void*)v2,a.size); close(fd2);
    munmap((void*)v,a.size); p.metadata=2;
    if(ioctl(fd,0xc028470c,&p) || ioctl(fd,0xc008470b,&a.offset)){perror("address free");return 56;}
    close(fd); puts("GPU_GOLDFISH_ALLOC_OK"); return 0;
}
struct waitcmd { uint32_t handle, flags; };
static int legacy_ping(int fd, uint32_t bo, uint32_t res, volatile uint32_t *response, uint32_t op, uint32_t size) {
    uint32_t words[12]={0x1003,res,0,0,size,0,op,0,0,0,0,0};
    struct exec e={.size=sizeof(words),.command=(uintptr_t)words,.handles=(uintptr_t)&bo,.num=1,.fd=-1};
    struct waitcmd wait={bo,0};
    if(ioctl(fd,0xc0286442,&e) || ioctl(fd,0xc0086448,&wait)){perror("legacy ping");return -1;}
    return (int)response[2];
}
static int legacy_asg(void) {
    int fd=open("/dev/dri/renderD128",O_RDWR); if(fd<0)return 30;
    struct create c={.target=0,.format=64,.bind=1<<17,.width=4096,.height=1,.depth=1,.size=4096,.stride=4096};
    if(ioctl(fd,0xc0386444,&c)){perror("legacy response create");return 31;}
    struct map m={.handle=c.bo}; if(ioctl(fd,0xc0106441,&m))return 32;
    volatile uint32_t *resp=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_SHARED,fd,m.offset);
    if(resp==MAP_FAILED)return 33;
    uint32_t create[2]={0x1001,10}; struct exec e={.size=8,.command=(uintptr_t)create,.fd=-1};
    if(ioctl(fd,0xc0286442,&e)){perror("legacy asg create");return 34;}
    int rings=legacy_ping(fd,c.bo,c.res,resp,0,0); if(rings<=0)return 35;
    int buffer=legacy_ping(fd,c.bo,c.res,resp,1,0); if(buffer<=0)return 36;
    if(legacy_ping(fd,c.bo,c.res,resp,2,1)!=1)return 37;
    uint64_t id=(uint64_t)resp[4] | ((uint64_t)resp[5]<<32);
    struct blob b={.mem=2,.flags=1,.size=(uint64_t)rings+buffer,.id=id};
    if(ioctl(fd,0xc030644a,&b)){perror("legacy ring blob");return 38;}
    struct map bm={.handle=b.bo}; if(ioctl(fd,0xc0106441,&bm))return 39;
    void *p=mmap(0,b.size,PROT_READ|PROT_WRITE,MAP_SHARED,fd,bm.offset);
    if(p==MAP_FAILED){perror("legacy ring mmap");return 40;}
    if(legacy_ping(fd,c.bo,c.res,resp,4,0)<0)return 41;
    munmap(p,b.size); munmap((void*)resp,4096); close(fd);
    printf("GPU_LEGACY_ASG_OK ring=%d buffer=%d\n",rings,buffer);
    return 0;
}
int main(void) {
  int address = address_allocator(); if (address) return address;
  int legacy = legacy_asg(); if (legacy) return legacy;
  int fd=open("/dev/dri/renderD128",O_RDWR); if(fd<0){perror("open");return 1;}
  struct param p={1,3}; struct init i={1,0,(uintptr_t)&p};
  if(ioctl(fd,0xc010644b,&i)){perror("context");return 2;}
  struct blob b={.mem=2,.flags=1,.size=4096,.id=0};
  if(ioctl(fd,0xc030644a,&b)){perror("blob");return 3;}
  struct map m={.handle=b.bo}; if(ioctl(fd,0xc0106441,&m)){perror("map ioctl");return 4;}
  volatile uint32_t *v=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_SHARED,fd,m.offset);
  if(v==MAP_FAILED){perror("mmap");return 5;}
  *v=0x12345678; printf("GPU_CONTEXT_BLOB_OK %x\n",*v);
  struct prime pr={.handle=b.bo,.flags=1};
  if(ioctl(fd,0xc00c642d,&pr)){perror("prime export");return 6;}
  volatile uint32_t *p2=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_SHARED,pr.fd,0);
  if(p2==MAP_FAILED || *p2!=*v){perror("prime map");return 7;}
  struct prime imp={.fd=pr.fd}; if(ioctl(fd,0xc00c642e,&imp)){perror("prime import");return 8;}
  struct create c={.target=0,.bind=1<<17,.width=4096,.height=1,.depth=1,.array_size=1,.size=4096};
  if(ioctl(fd,0xc0386444,&c)){perror("create3d");return 9;}
  struct map m2={.handle=c.bo}; if(ioctl(fd,0xc0106441,&m2)){perror("map3d");return 10;}
  volatile uint32_t *v2=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_SHARED,fd,m2.offset);
  if(v2==MAP_FAILED){perror("mmap3d");return 11;} *v2=0xabcdef01;
  struct create buffer={.target=0,.format=64,.bind=1<<22,.width=4096,.height=1,.depth=1,.array_size=1,.size=4096};
  if(ioctl(fd,0xc0386444,&buffer)){perror("buffer create");return 14;}
  struct map bm={.handle=buffer.bo}; if(ioctl(fd,0xc0106441,&bm)){perror("buffer map");return 15;}
  volatile uint32_t *bp=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_SHARED,fd,bm.offset);
  if(bp==MAP_FAILED){perror("buffer mmap");return 16;}
  *bp=0x76543210;
  struct transfer t={.handle=buffer.bo,.w=4,.h=1,.d=1};
  if(ioctl(fd,0xc02c6447,&t)){perror("transfer to host");return 17;}
  *bp=0;
  if(ioctl(fd,0xc02c6446,&t)){perror("transfer from host");return 18;}
  if(*bp!=0x76543210){printf("transfer mismatch %x\n",*bp);return 19;}
  puts("GPU_TRANSFER_OK");
  uint32_t command[3]={0xf002,0,0}; struct exec e={.flags=2,.size=sizeof(command),.command=(uintptr_t)command,.fd=-1};
  if(ioctl(fd,0xc0406442,&e)){perror("exec");return 12;}
  struct pollfd q={.fd=e.fd,.events=POLLIN};
  if(poll(&q,1,5000)!=1 || !(q.revents&POLLIN)){perror("fence poll");return 13;}
  struct exec e2={.flags=1|2|4,.size=sizeof(command),.command=(uintptr_t)command,.fd=e.fd,.ring=0};
  if(ioctl(fd,0xc0406442,&e2)){perror("exec input fence");return 20;}
  struct pollfd q2={.fd=e2.fd,.events=POLLIN};
  if(poll(&q2,1,5000)!=1 || !(q2.revents&POLLIN)){perror("ring fence poll");return 21;}
  close(e2.fd);
  printf("GPU_PRIME_RESOURCE_FENCE_OK %x\n",*v2);
  close(e.fd); munmap((void*)bp,4096); munmap((void*)v2,4096); munmap((void*)p2,4096); close(pr.fd);
  munmap((void*)v,4096); close(fd); return 0;
}
