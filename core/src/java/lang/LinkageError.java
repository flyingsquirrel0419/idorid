package java.lang;

public class LinkageError extends Error {
    public LinkageError() { super(); }
    public LinkageError(String m) { super(m); }
    public LinkageError(String m, Throwable c) { super(m, c); }
    public LinkageError(Throwable c) { super(c); }
}
