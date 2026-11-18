package java.util.concurrent;
import java.util.*;
public class ThreadPoolExecutor extends AbstractExecutorService {
 private boolean shutdown;private int pending;private final BlockingQueue<Runnable> queue;private ThreadFactory factory;private int core,max;private long keepAlive;private TimeUnit unit;
 public ThreadPoolExecutor(int core,int max,long keepAlive,TimeUnit unit,BlockingQueue<Runnable> queue){this(core,max,keepAlive,unit,queue,Executors.defaultThreadFactory());}
 public ThreadPoolExecutor(int core,int max,long keepAlive,TimeUnit unit,BlockingQueue<Runnable> queue,ThreadFactory factory){if(core<0||max<=0||max<core||keepAlive<0)throw new IllegalArgumentException();if(unit==null||queue==null||factory==null)throw new NullPointerException();this.core=core;this.max=max;this.keepAlive=keepAlive;this.unit=unit;this.queue=queue;this.factory=factory;}
 public void allowCoreThreadTimeOut(boolean value){if(value && keepAlive<=0)throw new IllegalArgumentException();}
 public void execute(final Runnable r){if(r==null)throw new NullPointerException();if(shutdown)throw new RejectedExecutionException();pending++;Thread t=factory.newThread(new Runnable(){public void run(){try{r.run();}finally{pending--;}}});if(t==null){pending--;throw new RejectedExecutionException();}t.start();}
 public void shutdown(){shutdown=true;}public List<Runnable> shutdownNow(){shutdown=true;ArrayList<Runnable> out=new ArrayList<Runnable>();queue.drainTo(out);return out;}public boolean isShutdown(){return shutdown;}public boolean isTerminated(){return shutdown&&pending==0;}public boolean awaitTermination(long n,TimeUnit u)throws InterruptedException{if(u==null)throw new NullPointerException();java.lang.VMThread.runPending();return isTerminated();}public BlockingQueue<Runnable> getQueue(){return queue;}public int getCorePoolSize(){return core;}public int getMaximumPoolSize(){return max;}public ThreadFactory getThreadFactory(){return factory;}
}
