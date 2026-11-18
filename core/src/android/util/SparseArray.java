package android.util;
public class SparseArray<E> implements Cloneable {
    private final java.util.TreeMap<Integer, E> map = new java.util.TreeMap<Integer, E>();
    public SparseArray() {}
    public SparseArray(int cap) {}
    public E get(int k) { return map.get(k); }
    public E get(int k, E d) { E v = map.get(k); return v == null ? d : v; }
    public void put(int k, E v) { map.put(k, v); }
    public void append(int k, E v) { map.put(k, v); }
    public void remove(int k) { map.remove(k); }
    public void delete(int k) { map.remove(k); }
    public void removeAt(int index) { map.remove(keyAt(index)); }
    public void setValueAt(int index,E value) { map.put(keyAt(index),value); }
    public int size() { return map.size(); }
    public void clear() { map.clear(); }
    public int keyAt(int i) { int n = 0; for (Integer k : map.keySet()) { if (n++ == i) return k; } throw new ArrayIndexOutOfBoundsException(i); }
    public E valueAt(int i) { return map.get(keyAt(i)); }
    public int indexOfKey(int k) { int n = 0; for (Integer x : map.keySet()) { if (x == k) return n; n++; } return -1; }
}
