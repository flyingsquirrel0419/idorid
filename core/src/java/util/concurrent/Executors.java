package java.util.concurrent;
import java.util.ArrayList;
import java.util.List;
public final class Executors {
 private Executors(){}
 public static ExecutorService newSingleThreadExecutor(){return new SimpleExecutor(1,defaultThreadFactory());}
 public static ExecutorService newFixedThreadPool(int n){return newFixedThreadPool(n,defaultThreadFactory());}
 public static ExecutorService newFixedThreadPool(int n,ThreadFactory f){if(n<=0)throw new IllegalArgumentException();if(f==null)throw new NullPointerException();return new SimpleExecutor(n,f);}
 public static ExecutorService newCachedThreadPool(){return new SimpleExecutor(Integer.MAX_VALUE,defaultThreadFactory());}
 public static ExecutorService newWorkStealingPool(){return newCachedThreadPool();} public static ExecutorService newWorkStealingPool(int n){return newFixedThreadPool(n);}
 public static Callable<Object> callable(final Runnable r){return callable(r,null);} public static <T> Callable<T> callable(final Runnable r,final T result){if(r==null)throw new NullPointerException();return new Callable<T>(){public T call(){r.run();return result;}};}
 public static ThreadFactory defaultThreadFactory(){return new ThreadFactory(){public Thread newThread(Runnable r){return new Thread(r);}};}
 public static ScheduledExecutorService newScheduledThreadPool(int n){return new ScheduledThreadPoolExecutor(n);} public static ScheduledExecutorService newScheduledThreadPool(int n,ThreadFactory f){return new ScheduledThreadPoolExecutor(n,f);} public static ScheduledExecutorService newSingleThreadScheduledExecutor(){return new ScheduledThreadPoolExecutor(1);} public static ScheduledExecutorService newSingleThreadScheduledExecutor(ThreadFactory f){return new ScheduledThreadPoolExecutor(1,f);}
 static class SimpleExecutor extends AbstractExecutorService {
  private boolean shutdown; private int running; private final int max; private final ThreadFactory factory; private final ArrayList<Runnable> queue=new ArrayList<Runnable>();
  SimpleExecutor(int max,ThreadFactory factory){this.max=max;this.factory=factory;}
  public synchronized void execute(Runnable r){if(r==null)throw new NullPointerException();if(shutdown)throw new RejectedExecutionException();queue.add(r);startWorkers();}
  private void startWorkers(){while(running<max&&!queue.isEmpty()){final Runnable task=queue.remove(0);Thread thread=factory.newThread(new Runnable(){public void run(){try{task.run();}finally{workerDone();}}});if(thread==null){queue.add(0,task);throw new RejectedExecutionException();}running++;thread.start();}}
  private synchronized void workerDone(){running--;startWorkers();}
  public synchronized void shutdown(){shutdown=true;} public synchronized List<Runnable> shutdownNow(){shutdown=true;ArrayList<Runnable> remaining=new ArrayList<Runnable>(queue);queue.clear();return remaining;}
  public synchronized boolean isShutdown(){return shutdown;} public synchronized boolean isTerminated(){return shutdown&&running==0&&queue.isEmpty();}
  public boolean awaitTermination(long n,TimeUnit u)throws InterruptedException{if(u==null)throw new NullPointerException();java.lang.VMThread.runPending();return isTerminated();}
 }
}
