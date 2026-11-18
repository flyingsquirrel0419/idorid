package java.lang;

public class Object {
    public Object() {}
    public final native Class<?> getClass();
    public native int hashCode();
    public boolean equals(Object o) { return this == o; }
    protected native Object clone() throws CloneNotSupportedException;
    public String toString() { return getClass().getName() + "@" + Integer.toHexString(hashCode()); }
    public final void notify() {}
    public final void notifyAll() {}
    public final void wait() throws InterruptedException {}
    public final void wait(long ms) throws InterruptedException {}
    public final void wait(long ms, int ns) throws InterruptedException {}
    protected void finalize() throws Throwable {}
}
