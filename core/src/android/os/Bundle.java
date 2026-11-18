package android.os;

import java.util.HashMap;
import java.util.Set;

public class Bundle implements Parcelable, Cloneable {
    public static final Bundle EMPTY = new Bundle();
    private final HashMap<String, Object> map = new HashMap<String, Object>();
    public Bundle() {}
    public Bundle(Bundle b) { if (b != null) map.putAll(b.map); }
    public Bundle(int cap) {}
    public boolean containsKey(String k) { return map.containsKey(k); }
    public Object get(String k) { return map.get(k); }
    public void remove(String k) { map.remove(k); }
    public Set<String> keySet() { return map.keySet(); }
    public int size() { return map.size(); }
    public boolean isEmpty() { return map.isEmpty(); }
    public void clear() { map.clear(); }
    public void putAll(Bundle b) { map.putAll(b.map); }
    public void putString(String k, String v) { map.put(k, v); }
    public void putInt(String k, int v) { map.put(k, v); }
    public void putLong(String k, long v) { map.put(k, v); }
    public void putBoolean(String k, boolean v) { map.put(k, v); }
    public void putFloat(String k, float v) { map.put(k, v); }
    public void putDouble(String k, double v) { map.put(k, v); }
    public void putChar(String k, char v) { map.put(k, v); }
    public void putCharSequence(String k, CharSequence v) { map.put(k, v); }
    public void putSerializable(String k, java.io.Serializable v) { map.put(k, v); }
    public void putParcelable(String k, Parcelable v) { map.put(k, v); }
    public void putBundle(String k, Bundle v) { map.put(k, v); }
    public void putStringArray(String k, String[] v) { map.put(k, v); }
    public void putIntArray(String k, int[] v) { map.put(k, v); }
    public void putStringArrayList(String k, java.util.ArrayList<String> v) { map.put(k, v); }
    public void putIntegerArrayList(String k, java.util.ArrayList<Integer> v) { map.put(k, v); }
    public String getString(String k) { Object o = map.get(k); return o instanceof String ? (String) o : null; }
    public String getString(String k, String d) { String s = getString(k); return s == null ? d : s; }
    public CharSequence getCharSequence(String k) { Object o = map.get(k); return o instanceof CharSequence ? (CharSequence) o : null; }
    public int getInt(String k) { return getInt(k, 0); }
    public int getInt(String k, int d) { Object o = map.get(k); return o instanceof Integer ? (Integer) o : d; }
    public long getLong(String k) { return getLong(k, 0L); }
    public long getLong(String k, long d) { Object o = map.get(k); return o instanceof Long ? (Long) o : d; }
    public boolean getBoolean(String k) { return getBoolean(k, false); }
    public boolean getBoolean(String k, boolean d) { Object o = map.get(k); return o instanceof Boolean ? (Boolean) o : d; }
    public float getFloat(String k) { return getFloat(k, 0f); }
    public float getFloat(String k, float d) { Object o = map.get(k); return o instanceof Float ? (Float) o : d; }
    public double getDouble(String k) { return getDouble(k, 0.0); }
    public double getDouble(String k, double d) { Object o = map.get(k); return o instanceof Double ? (Double) o : d; }
    public char getChar(String k) { Object o = map.get(k); return o instanceof Character ? (Character) o : 0; }
    public java.io.Serializable getSerializable(String k) { return (java.io.Serializable) map.get(k); }
    @SuppressWarnings("unchecked")
    public <T extends Parcelable> T getParcelable(String k) { return (T) map.get(k); }
    public Bundle getBundle(String k) { return (Bundle) map.get(k); }
    public String[] getStringArray(String k) { return (String[]) map.get(k); }
    public int[] getIntArray(String k) { return (int[]) map.get(k); }
    @SuppressWarnings("unchecked")
    public java.util.ArrayList<String> getStringArrayList(String k) { return (java.util.ArrayList<String>) map.get(k); }
    @SuppressWarnings("unchecked")
    public java.util.ArrayList<Integer> getIntegerArrayList(String k) { return (java.util.ArrayList<Integer>) map.get(k); }
    public void setClassLoader(ClassLoader loader) {}
    public <T> T getParcelable(String key, Class<T> type) { Object v=map.get(key); return type.isInstance(v) ? type.cast(v) : null; }
    public <T> T[] getParcelableArray(String key, Class<T> type) { return (T[])map.get(key); }
    public <T> java.util.ArrayList<T> getParcelableArrayList(String key, Class<? extends T> type) { return (java.util.ArrayList<T>)map.get(key); }
    public <T extends java.io.Serializable> T getSerializable(String key, Class<T> type) { Object v=map.get(key); return type.isInstance(v) ? type.cast(v) : null; }
    public <T extends Parcelable> void putSparseParcelableArray(String key, android.util.SparseArray<T> value) { map.put(key,value); }
    public <T extends Parcelable> android.util.SparseArray<T> getSparseParcelableArray(String key) { return (android.util.SparseArray<T>)map.get(key); }
    public <T> android.util.SparseArray<T> getSparseParcelableArray(String key, Class<? extends T> type) { return (android.util.SparseArray<T>)map.get(key); }
    public Object clone() { return new Bundle(this); }
    public int describeContents() { return 0; }
    public void writeToParcel(Parcel p, int f) {}
    public String toString() { return "Bundle[" + map + "]"; }
}
