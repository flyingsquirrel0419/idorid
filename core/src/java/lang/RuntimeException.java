package java.lang;

public class RuntimeException extends Exception {
    public RuntimeException() { super(); }
    public RuntimeException(String m) { super(m); }
    public RuntimeException(String m, Throwable c) { super(m, c); }
    public RuntimeException(Throwable c) { super(c); }
}
