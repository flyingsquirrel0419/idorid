package java.lang;

public class Error extends Throwable {
    public Error() { super(); }
    public Error(String m) { super(m); }
    public Error(String m, Throwable c) { super(m, c); }
    public Error(Throwable c) { super(c); }
}
