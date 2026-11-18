package java.util.concurrent;

public class RejectedExecutionException extends java.lang.RuntimeException {
    public RejectedExecutionException() { super(); }
    public RejectedExecutionException(String m) { super(m); }
    public RejectedExecutionException(String m, Throwable c) { super(m, c); }
    public RejectedExecutionException(Throwable c) { super(c); }
}
