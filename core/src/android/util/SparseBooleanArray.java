package android.util;
public class SparseBooleanArray implements Cloneable {
    private final SparseArray<Boolean> values=new SparseArray<Boolean>();
    public SparseBooleanArray() {} public SparseBooleanArray(int capacity) {}
    public boolean get(int key) { return get(key,false); } public boolean get(int key,boolean fallback) { Boolean v=values.get(key); return v==null?fallback:v.booleanValue(); }
    public void put(int key,boolean value) { values.put(key,Boolean.valueOf(value)); } public void append(int key,boolean value) { put(key,value); } public void delete(int key) { values.delete(key); } public void removeAt(int i) { values.removeAt(i); } public void clear() { values.clear(); }
    public int size() { return values.size(); } public int keyAt(int index) { return values.keyAt(index); } public boolean valueAt(int index) { return values.valueAt(index).booleanValue(); } public void setValueAt(int index,boolean value) { values.setValueAt(index,Boolean.valueOf(value)); } public int indexOfKey(int key) { return values.indexOfKey(key); } public int indexOfValue(boolean value) { for(int i=0;i<size();i++)if(valueAt(i)==value)return i;return -1; }
}
