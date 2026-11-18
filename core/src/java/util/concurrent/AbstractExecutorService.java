package java.util.concurrent;
public abstract class AbstractExecutorService implements ExecutorService {
 protected <T> RunnableFuture<T> newTaskFor(Runnable r,T v){return new FutureTask<T>(r,v);} protected <T> RunnableFuture<T> newTaskFor(Callable<T> c){return new FutureTask<T>(c);}
 public <T> Future<T> submit(Callable<T> c){if(c==null)throw new NullPointerException();RunnableFuture<T> f=newTaskFor(c);execute(f);return f;} public <T> Future<T> submit(Runnable r,T v){if(r==null)throw new NullPointerException();RunnableFuture<T> f=newTaskFor(r,v);execute(f);return f;} public Future<?> submit(Runnable r){return submit(r,null);}
 public <T> java.util.List<Future<T>> invokeAll(java.util.Collection<? extends Callable<T>> tasks)throws InterruptedException {java.util.ArrayList<Future<T>> out=new java.util.ArrayList<Future<T>>();for(Callable<T> c:tasks)out.add(submit(c));for(Future<T> f:out)try{f.get();}catch(ExecutionException e){}return out;}
 public <T> java.util.List<Future<T>> invokeAll(java.util.Collection<? extends Callable<T>> t,long n,TimeUnit u)throws InterruptedException {if(u==null)throw new NullPointerException();return invokeAll(t);}
 public <T> T invokeAny(java.util.Collection<? extends Callable<T>> tasks)throws InterruptedException,ExecutionException {if(tasks.isEmpty())throw new IllegalArgumentException();ExecutionException last=null;for(Callable<T> c:tasks)try{return submit(c).get();}catch(ExecutionException e){last=e;}throw last;}
 public <T> T invokeAny(java.util.Collection<? extends Callable<T>> t,long n,TimeUnit u)throws InterruptedException,ExecutionException,TimeoutException {if(u==null)throw new NullPointerException();return invokeAny(t);}
}
