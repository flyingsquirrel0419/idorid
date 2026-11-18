package java.lang;

public class IndexOutOfBoundsException extends RuntimeException {
    public IndexOutOfBoundsException() { super(); }
    public IndexOutOfBoundsException(String m) { super(m); }
    public IndexOutOfBoundsException(int i) { super("Index out of range: " + i); }
}
