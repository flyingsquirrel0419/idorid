package java.lang;

public class IllegalArgumentException extends RuntimeException {
    public IllegalArgumentException() { super(); }
    public IllegalArgumentException(String m) { super(m); }
    public IllegalArgumentException(String m, Throwable c) { super(m, c); }
    public IllegalArgumentException(Throwable c) { super(c); }
}
