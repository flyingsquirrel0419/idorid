package java.util;
public class LinkedHashSet<E> extends HashSet<E> implements Set<E>,Cloneable,java.io.Serializable {
 public LinkedHashSet(){super(16,.75f,true);}public LinkedHashSet(int capacity){super(capacity,.75f,true);}public LinkedHashSet(int capacity,float loadFactor){super(capacity,loadFactor,true);}public LinkedHashSet(Collection<? extends E> c){super(Math.max((int)(c.size()/.75f)+1,16),.75f,true);addAll(c);}
}
