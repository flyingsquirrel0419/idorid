package java.lang;

public class IllegalStateException extends RuntimeException {
    public IllegalStateException() { super(); }
    public IllegalStateException(String m) { super(m); }
    public IllegalStateException(String m, Throwable c) { super(m, c); }
    public IllegalStateException(Throwable c) { super(c); }
}
