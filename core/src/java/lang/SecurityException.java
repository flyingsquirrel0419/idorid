package java.lang;

public class SecurityException extends RuntimeException {
    public SecurityException() { super(); }
    public SecurityException(String m) { super(m); }
    public SecurityException(String m, Throwable c) { super(m, c); }
    public SecurityException(Throwable c) { super(c); }
}
