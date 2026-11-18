package java.lang;

public class NoSuchMethodException extends ReflectiveOperationException {
    public NoSuchMethodException() { super(); }
    public NoSuchMethodException(String m) { super(m); }
}
