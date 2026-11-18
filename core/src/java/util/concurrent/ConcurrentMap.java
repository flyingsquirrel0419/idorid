package java.util.concurrent;
import java.util.Map;
import java.util.Objects;
import java.util.function.*;
public interface ConcurrentMap<K,V> extends Map<K,V> {
 V putIfAbsent(K key,V value); boolean remove(Object key,Object value); boolean replace(K key,V oldValue,V newValue); V replace(K key,V value);
 default V getOrDefault(Object k,V d){V v=get(k);return v==null?d:v;}
 default void replaceAll(BiFunction<? super K,? super V,? extends V> f){Objects.requireNonNull(f);for(Entry<K,V> e:entrySet()){K k=e.getKey();V v=e.getValue();while(v!=null&&!replace(k,v,f.apply(k,v)))v=get(k);}}
 default V computeIfAbsent(K k,Function<? super K,? extends V> f){Objects.requireNonNull(f);V v=get(k);if(v==null){V n=f.apply(k);if(n!=null){v=putIfAbsent(k,n);if(v==null)v=n;}}return v;}
 default V computeIfPresent(K k,BiFunction<? super K,? super V,? extends V> f){Objects.requireNonNull(f);V old;while((old=get(k))!=null){V n=f.apply(k,old);if(n!=null){if(replace(k,old,n))return n;}else if(remove(k,old))return null;}return null;}
 default V compute(K k,BiFunction<? super K,? super V,? extends V> f){Objects.requireNonNull(f);for(;;){V old=get(k),n=f.apply(k,old);if(old==null){if(n==null)return null;if(putIfAbsent(k,n)==null)return n;}else if(n==null){if(remove(k,old))return null;}else if(replace(k,old,n))return n;}}
 default V merge(K k,V value,BiFunction<? super V,? super V,? extends V> f){Objects.requireNonNull(value);Objects.requireNonNull(f);for(;;){V old=get(k);if(old==null){if(putIfAbsent(k,value)==null)return value;}else {V n=f.apply(old,value);if(n==null){if(remove(k,old))return null;}else if(replace(k,old,n))return n;}}}
}
