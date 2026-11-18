package java.util.concurrent;

public class ExecutionException extends java.lang.Exception {
    public ExecutionException() { super(); }
    public ExecutionException(String m) { super(m); }
    public ExecutionException(String m, Throwable c) { super(m, c); }
    public ExecutionException(Throwable c) { super(c); }
}
