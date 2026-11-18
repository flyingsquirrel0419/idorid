package java.util;

public class ConcurrentModificationException extends java.lang.RuntimeException {
    public ConcurrentModificationException() { super(); }
    public ConcurrentModificationException(String m) { super(m); }
    public ConcurrentModificationException(String m, Throwable c) { super(m, c); }
    public ConcurrentModificationException(Throwable c) { super(c); }
}
