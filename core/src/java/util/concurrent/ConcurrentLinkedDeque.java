package java.util.concurrent;
import java.util.*;
public class ConcurrentLinkedDeque<E> extends AbstractCollection<E> implements Deque<E>, java.io.Serializable {
 private final LinkedList<E> q=new LinkedList<E>();
 public ConcurrentLinkedDeque(){} public ConcurrentLinkedDeque(Collection<? extends E> c){addAll(c);}
 private E checked(E e){if(e==null)throw new NullPointerException();return e;}
 public int size(){return q.size();} public Iterator<E> iterator(){return q.iterator();} public Iterator<E> descendingIterator(){return q.descendingIterator();}
 public void addFirst(E e){q.addFirst(checked(e));} public void addLast(E e){q.addLast(checked(e));} public boolean offerFirst(E e){addFirst(e);return true;} public boolean offerLast(E e){addLast(e);return true;}
 public boolean add(E e){addLast(e);return true;} public boolean offer(E e){return offerLast(e);} public E removeFirst(){return q.removeFirst();} public E removeLast(){return q.removeLast();} public E pollFirst(){return q.pollFirst();} public E pollLast(){return q.pollLast();} public E getFirst(){return q.getFirst();} public E getLast(){return q.getLast();} public E peekFirst(){return q.peekFirst();} public E peekLast(){return q.peekLast();}
 public boolean removeFirstOccurrence(Object o){return o!=null&&q.removeFirstOccurrence(o);} public boolean removeLastOccurrence(Object o){return o!=null&&q.removeLastOccurrence(o);} public boolean remove(Object o){return removeFirstOccurrence(o);} public E remove(){return removeFirst();} public E poll(){return pollFirst();} public E element(){return getFirst();} public E peek(){return peekFirst();} public void push(E e){addFirst(e);} public E pop(){return removeFirst();} public void clear(){q.clear();}
}