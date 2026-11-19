#include <pthread.h>
#include <stdio.h>
#include <stdatomic.h>
static pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cv = PTHREAD_COND_INITIALIZER;
static long counter; static int ready;
static atomic_long acount;
static void *worker(void *arg) {
  for (int i = 0; i < 20000; i++) { pthread_mutex_lock(&m); counter++; pthread_mutex_unlock(&m); atomic_fetch_add(&acount, 1); }
  pthread_mutex_lock(&m); ready++; pthread_cond_broadcast(&cv); pthread_mutex_unlock(&m);
  return (void*)(long)arg;
}
int main(void) {
  pthread_t t[4]; long sum = 0;
  for (long i = 0; i < 4; i++) pthread_create(&t[i], 0, worker, (void*)i);
  pthread_mutex_lock(&m); while (ready < 4) pthread_cond_wait(&cv, &m); pthread_mutex_unlock(&m);
  for (int i = 0; i < 4; i++) { void *r; pthread_join(t[i], &r); sum += (long)r; }
  printf("counter=%ld atomic=%ld sum=%ld\n", counter, (long)acount, sum);
  return 0;
}
