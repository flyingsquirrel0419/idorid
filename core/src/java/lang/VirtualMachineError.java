package java.lang;

public class VirtualMachineError extends Error {
    public VirtualMachineError() { super(); }
    public VirtualMachineError(String m) { super(m); }
    public VirtualMachineError(String m, Throwable c) { super(m, c); }
    public VirtualMachineError(Throwable c) { super(c); }
}
