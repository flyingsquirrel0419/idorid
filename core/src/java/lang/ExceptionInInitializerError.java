package java.lang;

public class ExceptionInInitializerError extends LinkageError {
    public ExceptionInInitializerError() { super(); }
    public ExceptionInInitializerError(String m) { super(m); }
    public ExceptionInInitializerError(String m, Throwable c) { super(m, c); }
    public ExceptionInInitializerError(Throwable c) { super(c); }
    public Throwable getException() { return getCause(); }
}
