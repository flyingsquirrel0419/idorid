package java.io;

/** fd 1 = stdout, 2 = stderr; other instances wrap an OutputStream. */
public class PrintStream extends OutputStream implements Appendable {
    private final int fd;
    private final OutputStream out;
    public PrintStream(int fd) { this.fd = fd; this.out = null; }
    public PrintStream(OutputStream out) { this.fd = 0; this.out = out; }
    public PrintStream(OutputStream out, boolean autoFlush) { this(out); }
    private static native void writeFd(int fd, String s);
    private void emit(String s) {
        if (out == null) { writeFd(fd, s); return; }
        try { out.write(s.getBytes()); } catch (IOException e) {}
    }
    public void write(int b) { if (out != null) { try { out.write(b); } catch (IOException e) {} } else emit(String.valueOf((char) b)); }
    public void print(String s) { emit(s == null ? "null" : s); }
    public void print(Object o) { emit(String.valueOf(o)); }
    public void print(char[] s) { emit(new String(s)); }
    public void print(char c) { emit(String.valueOf(c)); }
    public void print(int i) { emit(Integer.toString(i)); }
    public void print(long l) { emit(Long.toString(l)); }
    public void print(float f) { emit(Float.toString(f)); }
    public void print(double d) { emit(Double.toString(d)); }
    public void print(boolean b) { emit(b ? "true" : "false"); }
    public void println() { emit("\n"); }
    public void println(String s) { emit((s == null ? "null" : s) + "\n"); }
    public void println(Object o) { emit(String.valueOf(o) + "\n"); }
    public void println(char[] s) { emit(new String(s) + "\n"); }
    public void println(char c) { emit(String.valueOf(c) + "\n"); }
    public void println(int i) { emit(Integer.toString(i) + "\n"); }
    public void println(long l) { emit(Long.toString(l) + "\n"); }
    public void println(float f) { emit(Float.toString(f) + "\n"); }
    public void println(double d) { emit(Double.toString(d) + "\n"); }
    public void println(boolean b) { emit((b ? "true" : "false") + "\n"); }
    public PrintStream printf(String f, Object... a) { emit(String.format(f, a)); return this; }
    public PrintStream format(String f, Object... a) { return printf(f, a); }
    public PrintStream append(CharSequence s) { emit(String.valueOf(s)); return this; }
    public PrintStream append(CharSequence s, int b, int e) { emit(String.valueOf(s).substring(b, e)); return this; }
    public PrintStream append(char c) { emit(String.valueOf(c)); return this; }
    public void flush() {}
    public void close() {}
    public boolean checkError() { return false; }
}
