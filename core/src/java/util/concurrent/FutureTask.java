package java.util.concurrent;
public class FutureTask<V> implements RunnableFuture<V> {
 private Callable<V> callable; private V value; private Throwable failure; private boolean done, cancelled;
 public FutureTask(Callable<V> c){if(c==null)throw new NullPointerException();callable=c;}
 public FutureTask(final Runnable r,final V result){if(r==null)throw new NullPointerException();callable=new Callable<V>(){public V call(){r.run();return result;}};}
 public void run(){if(done)return; try{set(callable.call());}catch(Throwable t){setException(t);}}
 protected void set(V v){if(!done){value=v;done=true;done();}} protected void setException(Throwable t){if(!done){failure=t;done=true;done();}} protected void done(){} protected boolean runAndReset(){if(done)return false;try{callable.call();return true;}catch(Throwable t){setException(t);return false;}}
 public boolean cancel(boolean b){if(done)return false;cancelled=true;done=true;done();return true;} public boolean isCancelled(){return cancelled;} public boolean isDone(){return done;}
 public V get() throws InterruptedException,ExecutionException {java.lang.VMThread.runPending(); if(cancelled)throw new CancellationException();if(failure!=null)throw new ExecutionException(failure);return value;}
 public V get(long t,TimeUnit u) throws InterruptedException,ExecutionException,TimeoutException {if(u==null)throw new NullPointerException();java.lang.VMThread.runPending();if(!done)throw new TimeoutException();return get();}
}
