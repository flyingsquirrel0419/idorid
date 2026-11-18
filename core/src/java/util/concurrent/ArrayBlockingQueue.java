package java.util.concurrent;
import java.util.*;
public class ArrayBlockingQueue<E> extends LinkedBlockingQueue<E> { public ArrayBlockingQueue(int n){super(n);} public ArrayBlockingQueue(int n,boolean fair){super(n);} public ArrayBlockingQueue(int n,boolean fair,Collection<? extends E> c){super(n);addAll(c);} }
