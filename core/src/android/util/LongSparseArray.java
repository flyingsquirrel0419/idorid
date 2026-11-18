package android.util;
import java.util.TreeMap;
public class LongSparseArray<E> implements Cloneable {
    private final TreeMap<Long,E> values=new TreeMap<Long,E>();
    public LongSparseArray() {} public LongSparseArray(int capacity) {}
    public E get(long key) { return values.get(Long.valueOf(key)); } public E get(long key,E fallback) { Long k=Long.valueOf(key); return values.containsKey(k)?values.get(k):fallback; }
    public void put(long key,E value) { values.put(Long.valueOf(key),value); } public void append(long key,E value) { put(key,value); } public void delete(long key) { values.remove(Long.valueOf(key)); } public void remove(long key) { delete(key); } public void removeAt(int index) { delete(keyAt(index)); } public void clear() { values.clear(); }
    public int size() { return values.size(); } public long keyAt(int index) { int i=0; for(Long k:values.keySet()) { if(i++==index)return k.longValue(); } throw new ArrayIndexOutOfBoundsException(index); } public E valueAt(int index) { return get(keyAt(index)); } public void setValueAt(int index,E value) { put(keyAt(index),value); } public int indexOfKey(long key) { int i=0; for(Long k:values.keySet()) { if(k.longValue()==key)return i;i++; } return -1; } public int indexOfValue(E value) { for(int i=0;i<size();i++)if(valueAt(i)==value)return i;return -1; }
}
