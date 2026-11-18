import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
public class UtilConcurrent {
 public static void main(String[] args) throws Exception {
  AtomicInteger a=new AtomicInteger(4);
  System.out.println(a.getAndUpdate(x -> x+3));
  System.out.println(a.accumulateAndGet(2,(x,y)->x*y));
  AtomicReference<String> r=new AtomicReference<String>("a");
  System.out.println(r.updateAndGet(x->x+"b"));
  System.out.println(TimeUnit.SECONDS.toMillis(3));
  ExecutorService e=Executors.newSingleThreadExecutor();
  Future<Integer> f=e.submit(()->42);
  System.out.println(f.get());
  e.shutdown();
 }
}
