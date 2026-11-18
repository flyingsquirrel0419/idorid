package java.lang;

import java.io.PrintStream;

public class Throwable implements java.io.Serializable {
    private String detailMessage;
    private Throwable cause = this;
    /** VM-filled frames: "pkg.Class\u0000method\u0000File.java\u0000line" */
    private String[] vmTrace;
    private StackTraceElement[] stackTrace;
    private java.util.List<Throwable> suppressed;

    public Throwable() { fillInStackTrace(); }
    public Throwable(String message) { fillInStackTrace(); detailMessage = message; }
    public Throwable(String message, Throwable cause) { fillInStackTrace(); detailMessage = message; this.cause = cause; }
    public Throwable(Throwable cause) {
        fillInStackTrace();
        detailMessage = cause == null ? null : cause.toString();
        this.cause = cause;
    }
    protected Throwable(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        this(message, cause);
    }

    public native Throwable fillInStackTrace();

    public String getMessage() { return detailMessage; }
    public String getLocalizedMessage() { return getMessage(); }
    public synchronized Throwable getCause() { return cause == this ? null : cause; }
    public synchronized Throwable initCause(Throwable c) {
        if (cause != this) throw new IllegalStateException("Can't overwrite cause");
        if (c == this) throw new IllegalArgumentException("Self-causation not permitted");
        cause = c;
        return this;
    }
    public String toString() {
        String m = getLocalizedMessage();
        String n = getClass().getName();
        return m != null ? n + ": " + m : n;
    }
    public StackTraceElement[] getStackTrace() {
        if (stackTrace == null) {
            String[] t = vmTrace == null ? new String[0] : vmTrace;
            stackTrace = new StackTraceElement[t.length];
            for (int i = 0; i < t.length; i++) {
                String[] p = t[i].split("\u0000", -1);
                stackTrace[i] = new StackTraceElement(p[0], p[1], p.length > 2 && !p[2].isEmpty() ? p[2] : null, p.length > 3 ? Integer.parseInt(p[3]) : -1);
            }
        }
        return stackTrace.clone();
    }
    public void setStackTrace(StackTraceElement[] st) { stackTrace = st.clone(); }
    public final synchronized void addSuppressed(Throwable t) {
        if (suppressed == null) suppressed = new java.util.ArrayList<Throwable>();
        suppressed.add(t);
    }
    public final synchronized Throwable[] getSuppressed() {
        return suppressed == null ? new Throwable[0] : suppressed.toArray(new Throwable[0]);
    }
    public void printStackTrace() { printStackTrace(System.err); }
    public void printStackTrace(PrintStream s) {
        s.println(this);
        for (StackTraceElement e : getStackTrace()) s.println("\tat " + e);
        Throwable c = getCause();
        if (c != null && c != this) { s.print("Caused by: "); c.printStackTrace(s); }
    }
    public void printStackTrace(java.io.PrintWriter w) {
        w.println(this);
        for (StackTraceElement e : getStackTrace()) w.println("\tat " + e);
        Throwable c = getCause();
        if (c != null && c != this) { w.print("Caused by: "); c.printStackTrace(w); }
    }
}
