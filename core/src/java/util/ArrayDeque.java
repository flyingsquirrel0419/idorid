package java.util;

/** A resizable circular deque. Null elements are not permitted. */
public class ArrayDeque<E> extends AbstractCollection<E> implements Deque<E>, Cloneable, java.io.Serializable {
    private Object[] elements;
    private int head, size, modCount;
    public ArrayDeque(){elements=new Object[16];}
    public ArrayDeque(int n){elements=new Object[Math.max(1,n)];}
    public ArrayDeque(Collection<? extends E> c){this(c.size());addAll(c);}
    public int size(){return size;}
    private int index(int i){return (head+i)%elements.length;}
    private void grow(){Object[] a=new Object[elements.length+(elements.length>>1)+1];for(int i=0;i<size;i++)a[i]=elements[index(i)];elements=a;head=0;}
    public void addFirst(E e){Objects.requireNonNull(e);if(size==elements.length)grow();head=(head+elements.length-1)%elements.length;elements[head]=e;size++;modCount++;}
    public void addLast(E e){Objects.requireNonNull(e);if(size==elements.length)grow();elements[index(size++)]=e;modCount++;}
    public boolean offerFirst(E e){addFirst(e);return true;} public boolean offerLast(E e){addLast(e);return true;}
    public boolean add(E e){addLast(e);return true;} public boolean offer(E e){return offerLast(e);}
    public E peekFirst(){return size==0?null:(E)elements[head];} public E peekLast(){return size==0?null:(E)elements[index(size-1)];}
    public E getFirst(){E e=peekFirst();if(e==null)throw new NoSuchElementException();return e;} public E getLast(){E e=peekLast();if(e==null)throw new NoSuchElementException();return e;}
    public E pollFirst(){if(size==0)return null;E e=(E)elements[head];elements[head]=null;head=(head+1)%elements.length;size--;modCount++;return e;}
    public E pollLast(){if(size==0)return null;int i=index(--size);E e=(E)elements[i];elements[i]=null;modCount++;return e;}
    public E removeFirst(){E e=pollFirst();if(e==null)throw new NoSuchElementException();return e;} public E removeLast(){E e=pollLast();if(e==null)throw new NoSuchElementException();return e;}
    private void delete(int at){for(int i=at;i<size-1;i++)elements[index(i)]=elements[index(i+1)];elements[index(size-1)]=null;size--;modCount++;}
    public boolean removeFirstOccurrence(Object o){if(o!=null)for(int i=0;i<size;i++)if(o.equals(elements[index(i)])){delete(i);return true;}return false;}
    public boolean removeLastOccurrence(Object o){if(o!=null)for(int i=size-1;i>=0;i--)if(o.equals(elements[index(i)])){delete(i);return true;}return false;}
    public boolean remove(Object o){return removeFirstOccurrence(o);} public void clear(){if(size!=0){for(int i=0;i<size;i++)elements[index(i)]=null;size=0;head=0;modCount++;}}
    public E remove(){return removeFirst();} public E poll(){return pollFirst();} public E element(){return getFirst();} public E peek(){return peekFirst();} public void push(E e){addFirst(e);} public E pop(){return removeFirst();}
    public Iterator<E> iterator(){return iter(false);} public Iterator<E> descendingIterator(){return iter(true);}
    private Iterator<E> iter(final boolean reverse){return new Iterator<E>(){int cursor=reverse?size-1:0,last=-1,expected=modCount;public boolean hasNext(){return reverse?cursor>=0:cursor<size;}public E next(){if(expected!=modCount)throw new ConcurrentModificationException();if(!hasNext())throw new NoSuchElementException();last=cursor;E e=(E)elements[index(cursor)];cursor+=reverse?-1:1;return e;}public void remove(){if(expected!=modCount)throw new ConcurrentModificationException();if(last<0)throw new IllegalStateException();delete(last);if(!reverse)cursor--;last=-1;expected=modCount;}};}
}
