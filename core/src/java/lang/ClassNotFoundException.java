package java.lang;

public class ClassNotFoundException extends ReflectiveOperationException {
    public ClassNotFoundException() { super(); }
    public ClassNotFoundException(String m) { super(m); }
    public ClassNotFoundException(String m, Throwable c) { super(m, c); }
    public ClassNotFoundException(Throwable c) { super(c); }
}
