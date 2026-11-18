package android.os;
import java.util.ArrayList;
import java.util.List;
public final class Parcel {
    private final ArrayList<Object> data=new ArrayList<Object>(); private int position;
    private Parcel() {} public static Parcel obtain() { return new Parcel(); }
    public void recycle() { data.clear(); position=0; }
    public int dataPosition() { return position; } public int dataSize() { return data.size(); }
    public void setDataPosition(int p) { if (p<0 || p>data.size()) throw new IndexOutOfBoundsException(); position=p; }
    private void put(Object o) { if (position==data.size()) data.add(o); else data.set(position,o); position++; }
    private Object take() { return data.get(position++); }
    public void writeString(String v) { put(v); } public String readString() { return (String)take(); }
    public void writeInt(int v) { put(Integer.valueOf(v)); } public int readInt() { return ((Integer)take()).intValue(); }
    public void writeLong(long v) { put(Long.valueOf(v)); } public long readLong() { return ((Long)take()).longValue(); }
    public void writeByte(byte v) { writeInt(v); } public byte readByte() { return (byte)readInt(); }
    public void writeFloat(float v) { put(Float.valueOf(v)); } public float readFloat() { return ((Float)take()).floatValue(); }
    public void writeDouble(double v) { put(Double.valueOf(v)); } public double readDouble() { return ((Double)take()).doubleValue(); }
    public void writeValue(Object o) { put(o); } public Object readValue(ClassLoader loader) { return take(); }
    public void writeBundle(Bundle b) { put(b==null?null:new Bundle(b)); } public Bundle readBundle() { return (Bundle)take(); } public Bundle readBundle(ClassLoader loader) { return readBundle(); }
    public void writeParcelable(Parcelable p,int flags) { put(p); } public <T extends Parcelable> T readParcelable(ClassLoader loader) { return (T)take(); }
    public void writeParcelableArray(Parcelable[] a,int flags) { put(a==null?null:a.clone()); } public Parcelable[] readParcelableArray(ClassLoader loader) { return (Parcelable[])take(); }
    public void writeByteArray(byte[] a) { put(a==null?null:a.clone()); } public byte[] createByteArray() { return (byte[])take(); }
    public void readByteArray(byte[] out) { byte[] a=createByteArray(); if (a.length!=out.length) throw new RuntimeException(); System.arraycopy(a,0,out,0,a.length); }
    public void writeIntArray(int[] a) { put(a==null?null:a.clone()); } public int[] createIntArray() { return (int[])take(); }
    public void readIntArray(int[] out) { int[] a=createIntArray(); if (a.length!=out.length) throw new RuntimeException(); System.arraycopy(a,0,out,0,a.length); }
    public void writeStringArray(String[] a) { put(a==null?null:a.clone()); } public String[] createStringArray() { return (String[])take(); }
    public void writeList(List list) { put(list==null?null:new ArrayList(list)); } public ArrayList readArrayList(ClassLoader loader) { return (ArrayList)take(); }
    public void writeStringList(List<String> list) { writeList(list); } public ArrayList<String> createStringArrayList() { return (ArrayList<String>)readArrayList(null); }
    public <T extends Parcelable> void writeTypedList(List<T> list) { if (list==null) { writeInt(-1); return; } writeInt(list.size()); for (T p:list) { writeInt(p==null?0:1); if (p!=null) p.writeToParcel(this,0); } }
    public <T> ArrayList<T> createTypedArrayList(Parcelable.Creator<T> creator) { int n=readInt(); if (n<0) return null; ArrayList<T> result=new ArrayList<T>(); for (int i=0;i<n;i++) result.add(readInt()==0?null:creator.createFromParcel(this)); return result; }
    public <T extends Parcelable> void writeTypedArray(T[] array,int flags) { if (array==null) { writeInt(-1); return; } writeInt(array.length); for (T p:array) { writeInt(p==null?0:1); if (p!=null) p.writeToParcel(this,flags); } }
    public <T> T[] createTypedArray(Parcelable.Creator<T> creator) { int n=readInt(); if (n<0) return null; T[] array=creator.newArray(n); for (int i=0;i<n;i++) if (readInt()!=0) array[i]=creator.createFromParcel(this); return array; }
}
