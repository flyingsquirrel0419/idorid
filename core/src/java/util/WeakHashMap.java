package java.util;
public class WeakHashMap<K,V> extends HashMap<K,V> {public WeakHashMap(){super();}public WeakHashMap(int capacity){super(capacity);}public WeakHashMap(int capacity,float loadFactor){super(capacity,loadFactor);}public WeakHashMap(Map<? extends K,? extends V> m){super(m);}}
