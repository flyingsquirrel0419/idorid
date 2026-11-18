package java.lang.reflect;

public class InvocationTargetException extends java.lang.ReflectiveOperationException {
    public InvocationTargetException() { super(); }
    public InvocationTargetException(String m) { super(m); }
    public InvocationTargetException(String m, Throwable c) { super(m, c); }
    public InvocationTargetException(Throwable c) { super(c); }
    public Throwable getTargetException() { return getCause(); }
}
