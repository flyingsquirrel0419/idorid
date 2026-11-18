package java.lang.ref;
public class SoftReference<T> extends Reference<T> {
    public SoftReference(T r) { super(r); }
    public SoftReference(T r, ReferenceQueue<? super T> q) { super(r); }
}
