package java.lang;

public class InternalError extends VirtualMachineError {
    public InternalError() { super(); }
    public InternalError(String m) { super(m); }
    public InternalError(String m, Throwable c) { super(m, c); }
    public InternalError(Throwable c) { super(c); }
}
