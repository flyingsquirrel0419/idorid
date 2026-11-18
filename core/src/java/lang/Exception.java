package java.lang;

public class Exception extends Throwable {
    public Exception() { super(); }
    public Exception(String m) { super(m); }
    public Exception(String m, Throwable c) { super(m, c); }
    public Exception(Throwable c) { super(c); }
}
