package java.util.concurrent;
import java.util.*;
public class ConcurrentSkipListSet<E> extends TreeSet<E> implements Cloneable,java.io.Serializable {
 public ConcurrentSkipListSet(){super();}public ConcurrentSkipListSet(Comparator<? super E> c){super(c);}public ConcurrentSkipListSet(Collection<? extends E> c){super();addAll(c);}public ConcurrentSkipListSet(SortedSet<E> s){super(s);}public boolean add(E e){if(e==null)throw new NullPointerException();return super.add(e);}
}
