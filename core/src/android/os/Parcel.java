package android.os;
public final class Parcel {
    private Parcel() {}
    public static Parcel obtain() { return new Parcel(); }
    public void recycle() {}
    public void writeString(String s) {}
    public void writeInt(int i) {}
    public String readString() { return null; }
    public int readInt() { return 0; }
}
