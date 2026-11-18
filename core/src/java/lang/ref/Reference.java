package java.lang.ref;
/** Strong references (no reference processing yet). */
public abstract class Reference<T> {
    private T referent;
    Reference(T r) { referent = r; }
    public T get() { return referent; }
    public void clear() { referent = null; }
    public boolean enqueue() { return false; }
    public boolean isEnqueued() { return false; }
}
