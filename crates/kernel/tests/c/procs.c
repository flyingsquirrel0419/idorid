#define _GNU_SOURCE
#include <stdio.h>
#include <unistd.h>
#include <sys/wait.h>
#include <signal.h>
#include <string.h>
#include <stdlib.h>
#include <sys/mman.h>
static volatile int got;
static void h(int s, siginfo_t *si, void *uc) { got = s; }
int main(void) {
  int p[2]; pipe(p);
  pid_t c = fork();
  if (c == 0) { close(p[0]); const char *m = "child says hi"; write(p[1], m, strlen(m)); _exit(7); }
  close(p[1]); char buf[64] = {0}; int n = read(p[0], buf, sizeof buf - 1);
  int st; waitpid(c, &st, 0);
  printf("read %d '%s' exit=%d\n", n, buf, WEXITSTATUS(st));
  struct sigaction sa = {0}; sa.sa_sigaction = h; sa.sa_flags = SA_SIGINFO; sigaction(SIGUSR1, &sa, 0);
  raise(SIGUSR1); printf("handler got %d\n", got);
  // shared anonymous memory across fork
  int *sh = mmap(0, 4096, PROT_READ|PROT_WRITE, MAP_SHARED|MAP_ANONYMOUS, -1, 0);
  int *pr = malloc(sizeof(int)); *pr = 1; *sh = 1;
  if ((c = fork()) == 0) { *sh = 42; *pr = 99; _exit(0); }
  waitpid(c, &st, 0);
  printf("shared=%d private=%d\n", *sh, *pr);
  // SIGSEGV handler with siglongjmp-free recovery: catch and exit
  char *big = mmap(0, 1<<20, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
  big[12345] = 5; big = mremap(big, 1<<20, 2<<20, MREMAP_MAYMOVE); printf("mremap kept %d\n", big[12345]);
  if ((c = fork()) == 0) { char *e[] = {0}; char *a[] = {"/bin/x", 0}; execve("/nonexistent", a, e); _exit(3); }
  waitpid(c, &st, 0); printf("exec fail exit=%d\n", WEXITSTATUS(st));
  if ((c = fork()) == 0) { *(volatile int*)0 = 1; _exit(0); }
  waitpid(c, &st, 0); printf("segv child signaled=%d sig=%d\n", WIFSIGNALED(st), WTERMSIG(st));
  return 0;
}
