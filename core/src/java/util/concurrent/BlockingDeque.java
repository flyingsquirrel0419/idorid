package java.util.concurrent;
import java.util.*;
public interface BlockingDeque<E> extends BlockingQueue<E>,Deque<E> {void putFirst(E e)throws InterruptedException;void putLast(E e)throws InterruptedException;boolean offerFirst(E e,long timeout,TimeUnit u)throws InterruptedException;boolean offerLast(E e,long timeout,TimeUnit u)throws InterruptedException;E takeFirst()throws InterruptedException;E takeLast()throws InterruptedException;E pollFirst(long timeout,TimeUnit u)throws InterruptedException;E pollLast(long timeout,TimeUnit u)throws InterruptedException;}
