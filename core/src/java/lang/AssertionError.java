package java.lang;
public class AssertionError extends Error {
    public AssertionError() {}
    public AssertionError(Object detail) { super(String.valueOf(detail)); if (detail instanceof Throwable) initCause((Throwable) detail); }
    public AssertionError(String m, Throwable c) { super(m, c); }
    public AssertionError(boolean b) { this(String.valueOf(b)); }
    public AssertionError(int i) { this(String.valueOf(i)); }
    public AssertionError(long l) { this(String.valueOf(l)); }
}
