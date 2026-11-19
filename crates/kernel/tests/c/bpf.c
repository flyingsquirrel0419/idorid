#include <linux/bpf.h>
#include <sys/syscall.h>
#include <sys/wait.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <fcntl.h>
#include <stdlib.h>
#define CHECK(x) do { if (!(x)) { printf("FAIL line %d errno %d\n",__LINE__,errno); exit(1); } } while (0)
static int call(int cmd, union bpf_attr *a) { return syscall(__NR_bpf,cmd,a,sizeof(*a)); }
static int elem(int cmd,int fd, uint32_t *key, void *val, uint64_t flags) {union bpf_attr a={0};a.map_fd=fd;a.key=(uintptr_t)key;a.value=(uintptr_t)val;a.flags=flags;return call(cmd,&a);}
int main(void) {
 union bpf_attr a={0}; a.map_type=BPF_MAP_TYPE_HASH;a.key_size=4;a.value_size=4;a.max_entries=1;memcpy(a.map_name,"testmap",7); int fd=call(BPF_MAP_CREATE,&a);CHECK(fd>=0);
 uint32_t k=7,k2=8,v=42,out=0; CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&out,0)<0 && errno==ENOENT);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&v,BPF_NOEXIST)==0);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&v,BPF_NOEXIST)<0 && errno==EEXIST);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k2,&v,BPF_ANY)<0 && errno==E2BIG);CHECK(elem(BPF_MAP_GET_NEXT_KEY,fd,NULL,&out,0)==0 && out==k);
 memset(&a,0,sizeof(a));a.pathname=(uintptr_t)"/sys/fs/bpf/testmap";a.bpf_fd=fd;CHECK(call(BPF_OBJ_PIN,&a)==0); CHECK(call(BPF_OBJ_PIN,&a)<0 && errno==EEXIST); CHECK(rename("/sys/fs/bpf/testmap", "/sys/fs/bpf/renamed") == 0); close(fd); memset(&a,0,sizeof(a));a.pathname=(uintptr_t)"/sys/fs/bpf/renamed";a.file_flags=BPF_F_RDONLY; fd=call(BPF_OBJ_GET,&a); CHECK(fd>=0); CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&out,0)==0 && out==42);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&v,0)<0 && errno==EPERM);
 struct bpf_map_info info={0};memset(&a,0,sizeof(a));a.info.bpf_fd=fd;a.info.info_len=sizeof(info);a.info.info=(uintptr_t)&info;CHECK(call(BPF_OBJ_GET_INFO_BY_FD,&a)==0 && info.type==BPF_MAP_TYPE_HASH && info.key_size==4 && info.value_size==4 && info.max_entries==1 && info.id && !strcmp(info.name,"testmap"));close(fd);
 memset(&a,0,sizeof(a));a.pathname=(uintptr_t)"/sys/fs/bpf/renamed";fd=call(BPF_OBJ_GET,&a);CHECK(fd>=0);pid_t child=fork(); CHECK(child>=0); if(child==0) { uint32_t child_value=77; _exit(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&child_value,BPF_EXIST)==0 ? 0 : 1); } int status=0; CHECK(waitpid(child,&status,0)==child && WIFEXITED(status) && WEXITSTATUS(status)==0); CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&out,0)==0 && out==77);CHECK(elem(BPF_MAP_DELETE_ELEM,fd,&k,NULL,0)==0);CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&out,0)<0 && errno==ENOENT);close(fd);
 memset(&a,0,sizeof(a));a.map_type=BPF_MAP_TYPE_PERCPU_ARRAY;a.key_size=4;a.value_size=4;a.max_entries=2;fd=call(BPF_MAP_CREATE,&a);CHECK(fd>=0);k=0;uint64_t pv=0x1122334455667788ULL,pout=0;CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&pout,0)==0 && pout==0);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&pv,0)==0);CHECK(elem(BPF_MAP_LOOKUP_ELEM,fd,&k,&pout,0)==0 && pout==pv); out=99; CHECK(elem(BPF_MAP_GET_NEXT_KEY,fd,&k,&out,0)==0 && out==1); k=1; CHECK(elem(BPF_MAP_GET_NEXT_KEY,fd,&k,&out,0)<0 && errno==ENOENT); k=0; memset(&a,0,sizeof(a));a.map_fd=fd;CHECK(call(BPF_MAP_FREEZE,&a)==0);CHECK(elem(BPF_MAP_UPDATE_ELEM,fd,&k,&pv,0)<0 && errno==EPERM);
 memset(&a,0,sizeof(a));a.prog_type=BPF_PROG_TYPE_SOCKET_FILTER;a.license=(uintptr_t)"GPL";memcpy(a.prog_name,"testprog",8);int prog=call(BPF_PROG_LOAD,&a);CHECK(prog>=0);struct bpf_prog_info pi={0};memset(&a,0,sizeof(a));a.info.bpf_fd=prog;a.info.info_len=sizeof(pi);a.info.info=(uintptr_t)&pi;CHECK(call(BPF_OBJ_GET_INFO_BY_FD,&a)==0 && pi.type==BPF_PROG_TYPE_SOCKET_FILTER && pi.id && !strcmp(pi.name,"testprog"));memset(&a,0,sizeof(a)); CHECK(call(BPF_PROG_ATTACH,&a)==0 && call(BPF_PROG_DETACH,&a)==0); a.query.prog_cnt=5; CHECK(call(BPF_PROG_QUERY,&a)==0 && a.query.prog_cnt==0);close(prog);
 puts("bpf tests passed");return 0;
}
