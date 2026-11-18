package java.util;
public class HashSet<E> extends AbstractSet<E> implements Cloneable, java.io.Serializable {
    final HashMap<E,Object> map;
    private static final Object PRESENT = new Object();
    HashSet(int capacity,float loadFactor,boolean linked){map=new LinkedHashMap<E,Object>(capacity,loadFactor);}
    public HashSet(){map=new HashMap<E,Object>();}
    public HashSet(int capacity){map=new HashMap<E,Object>(capacity);}
    public HashSet(int capacity,float loadFactor){map=new HashMap<E,Object>(capacity,loadFactor);}
    public HashSet(Collection<? extends E> c){this(Math.max((int)(c.size()/.75f)+1,16));addAll(c);}
    public int size(){return map.size();}
    public boolean isEmpty(){return map.isEmpty();}
    public boolean contains(Object o){return map.containsKey(o);}
    public boolean add(E e){return map.put(e,PRESENT)==null;}
    public boolean remove(Object o){return map.remove(o)==PRESENT;}
    public void clear(){map.clear();}
    public Iterator<E> iterator(){return map.keySet().iterator();}
}
