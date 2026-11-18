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
  ConcurrentHashMap<String,Integer> map=new ConcurrentHashMap<String,Integer>();
  map.put("a",1); map.merge("a",2,(x,y)->x+y);
  System.out.println(map.get("a"));
  AtomicIntegerArray ar=new AtomicIntegerArray(new int[]{2,3});
  System.out.println(ar.updateAndGet(1,x->x+4));
  System.out.println(CompletableFuture.allOf(CompletableFuture.completedFuture(1),CompletableFuture.completedFuture(2)).join());
  System.out.println(CompletableFuture.supplyAsync(() -> 7).thenApply(x -> x * 3).join());
  System.out.println(CompletableFuture.completedFuture(8).thenCompose(x -> CompletableFuture.completedFuture(x + 4)).join());
  System.out.println(CompletableFuture.completedFuture(5).thenCombine(CompletableFuture.completedFuture(6), (x,y) -> x+y).join());
 }
}
