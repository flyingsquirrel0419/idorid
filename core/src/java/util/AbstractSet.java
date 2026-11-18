package java.util;
public abstract class AbstractSet<E> extends AbstractCollection<E> implements Set<E> { public boolean equals(Object o){if(o==this)return true;if(!(o instanceof Set))return false;Set<?> s=(Set<?>)o;return s.size()==size()&&containsAll(s);} public int hashCode(){int h=0;for(E e:this)h+=Objects.hashCode(e);return h;} }
