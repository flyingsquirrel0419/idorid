import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.locks.*;
public class UtilConcurrent2 {
 public static void main(String[] args) throws Exception {
  ConcurrentLinkedQueue<String> q=new ConcurrentLinkedQueue<String>();q.add("one");q.add("two");System.out.println(q+" "+q.poll()+" "+q.peek());
  ConcurrentLinkedDeque<Integer> d=new ConcurrentLinkedDeque<Integer>();d.addFirst(2);d.addFirst(1);d.addLast(3);System.out.println(d+" "+d.removeLast());
  CopyOnWriteArrayList<String> a=new CopyOnWriteArrayList<String>();a.add("a");Iterator<String> it=a.iterator();a.add("b");System.out.println(it.next()+" "+it.hasNext()+" "+a);
  CopyOnWriteArraySet<Integer> s=new CopyOnWriteArraySet<Integer>();s.add(3);s.add(1);s.add(3);System.out.println(s);
  ArrayBlockingQueue<Integer> b=new ArrayBlockingQueue<Integer>(2);b.add(4);b.add(5);System.out.println(b.offer(6)+" "+b.remainingCapacity()+" "+b.poll());
  PriorityBlockingQueue<Integer> p=new PriorityBlockingQueue<Integer>();p.add(9);p.add(2);p.add(4);System.out.println(p.poll()+" "+p.poll());
  ConcurrentSkipListMap<Integer,String> m=new ConcurrentSkipListMap<Integer,String>();m.put(3,"c");m.put(1,"a");m.put(2,"b");System.out.println(m+" "+m.floorKey(2)+" "+m.higherKey(2));
  Semaphore permits=new Semaphore(2);System.out.println(permits.tryAcquire(2)+" "+permits.tryAcquire()+" "+permits.availablePermits());permits.release();System.out.println(permits.availablePermits());
  Executor direct=new Executor(){public void execute(Runnable r){r.run();}};
  CompletableFuture<Integer> f=CompletableFuture.completedFuture(4).thenApplyAsync(x -> x+1,direct);System.out.println(f.join()+" "+f.thenCombine(CompletableFuture.completedFuture(3),(x,y)->x+y).join());
  ReentrantLock lock=new ReentrantLock();lock.lock();lock.lock();System.out.println(lock.isLocked()+" "+lock.getHoldCount());lock.unlock();lock.unlock();System.out.println(lock.isLocked());
 }
}
