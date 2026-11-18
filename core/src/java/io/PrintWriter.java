package java.io;
public class PrintWriter extends Writer {
    protected Writer out;
    public PrintWriter(Writer w) { out = w; }
    public PrintWriter(Writer w, boolean af) { out = w; }
    public PrintWriter(OutputStream os) { out = null; stream = os; }
    private OutputStream stream;
    public void write(char[] c, int off, int len) {
        try { if (out != null) out.write(c, off, len); else stream.write(new String(c, off, len).getBytes()); } catch (IOException e) {}
    }
    public void write(String s) { write(s.toCharArray(), 0, s.length()); }
    public void print(String s) { write(String.valueOf(s)); }
    public void print(Object o) { write(String.valueOf(o)); }
    public void print(boolean b) { write(String.valueOf(b)); }
    public void println(boolean b) { write(String.valueOf(b)+"\n"); }
    public void print(int i) { write(String.valueOf(i)); }
    public void print(char c) { write(String.valueOf(c)); }
    public void println() { write("\n"); }
    public void println(String s) { write(s + "\n"); }
    public void println(Object o) { write(String.valueOf(o) + "\n"); }
    public void println(int i) { write(i + "\n"); }
    public PrintWriter printf(String f, Object... a) { write(String.format(f, a)); return this; }
    public PrintWriter format(String f, Object... a) { return printf(f, a); }
    public void flush() {}
    public void close() {}
}
