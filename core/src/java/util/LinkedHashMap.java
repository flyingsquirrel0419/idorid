package java.util;

public class LinkedHashMap<K,V> extends HashMap<K,V> implements Map<K,V> {
 private static class Link<K,V> extends AbstractMap.SimpleEntry<K,V> {Link<K,V> before,after;Link(K k,V v){super(k,v);}}
 private final HashMap<K,Link<K,V>> links=new HashMap<K,Link<K,V>>();
 private Link<K,V> first,last;private int changes;private final boolean accessOrder;
 public LinkedHashMap(){super();accessOrder=false;} public LinkedHashMap(int n){super(n);accessOrder=false;} public LinkedHashMap(int n,float f){super(n,f);accessOrder=false;} public LinkedHashMap(int n,float f,boolean accessOrder){super(n,f);this.accessOrder=accessOrder;} public LinkedHashMap(Map<? extends K,? extends V> m){this();putAll(m);}
 private void append(Link<K,V> e){e.before=last;e.after=null;if(last==null)first=e;else last.after=e;last=e;}
 private void unlink(Link<K,V> e){if(e.before==null)first=e.after;else e.before.after=e.after;if(e.after==null)last=e.before;else e.after.before=e.before;}
 private void access(Link<K,V> e){if(accessOrder&&e!=last){unlink(e);append(e);changes++;}}
 public int size(){return links.size();} public boolean isEmpty(){return links.isEmpty();}public boolean containsKey(Object k){return links.containsKey(k);}public V get(Object k){Link<K,V> e=links.get(k);if(e==null)return null;access(e);return e.getValue();}
 public V getOrDefault(Object k,V d){Link<K,V> e=links.get(k);if(e==null)return d;access(e);return e.getValue();}
 public V put(K k,V v){Link<K,V> e=links.get(k);if(e!=null){V old=e.setValue(v);access(e);return old;}e=new Link<K,V>(k,v);links.put(k,e);append(e);changes++;if(removeEldestEntry(first))remove(first.getKey());return null;}
 protected boolean removeEldestEntry(Map.Entry<K,V> eldest){return false;}
 public V remove(Object k){Link<K,V> e=links.remove(k);if(e==null)return null;unlink(e);changes++;return e.getValue();}public void clear(){if(!links.isEmpty()){links.clear();first=last=null;changes++;}}
 public Set<Map.Entry<K,V>> entrySet(){return new AbstractSet<Map.Entry<K,V>>(){public int size(){return links.size();}public void clear(){LinkedHashMap.this.clear();}public Iterator<Map.Entry<K,V>> iterator(){return new Iterator<Map.Entry<K,V>>(){Link<K,V> next=first,current;int expected=changes;public boolean hasNext(){return next!=null;}public Map.Entry<K,V> next(){if(expected!=changes)throw new ConcurrentModificationException();if(next==null)throw new NoSuchElementException();current=next;next=next.after;return current;}public void remove(){if(expected!=changes)throw new ConcurrentModificationException();if(current==null)throw new IllegalStateException();LinkedHashMap.this.remove(current.getKey());current=null;expected=changes;}};}};}
}
