package android.util;
public class SparseIntArray implements Cloneable {
    private final SparseArray<Integer> values=new SparseArray<Integer>();
    public SparseIntArray() {} public SparseIntArray(int capacity) {}
    public int get(int key) { return get(key,0); } public int get(int key,int fallback) { Integer v=values.get(key); return v==null?fallback:v.intValue(); }
    public void put(int key,int value) { values.put(key,Integer.valueOf(value)); } public void append(int key,int value) { put(key,value); } public void delete(int key) { values.delete(key); } public void removeAt(int i) { values.removeAt(i); } public void clear() { values.clear(); }
    public int size() { return values.size(); } public int keyAt(int index) { return values.keyAt(index); } public int valueAt(int index) { return values.valueAt(index).intValue(); } public void setValueAt(int index,int value) { values.setValueAt(index,Integer.valueOf(value)); } public int indexOfKey(int key) { return values.indexOfKey(key); } public int indexOfValue(int value) { for(int i=0;i<size();i++)if(valueAt(i)==value)return i;return -1; }
}
