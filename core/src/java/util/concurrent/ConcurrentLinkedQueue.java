package java.util.concurrent;
import java.util.*;
public class ConcurrentLinkedQueue<E> extends AbstractQueue<E> implements java.io.Serializable {
 private final ConcurrentLinkedDeque<E> q=new ConcurrentLinkedDeque<E>();
 public ConcurrentLinkedQueue(){} public ConcurrentLinkedQueue(Collection<? extends E> c){addAll(c);} public int size(){return q.size();} public Iterator<E> iterator(){return q.iterator();} public boolean offer(E e){return q.offer(e);} public E poll(){return q.poll();} public E peek(){return q.peek();} public boolean remove(Object o){return q.remove(o);} public void clear(){q.clear();}
}