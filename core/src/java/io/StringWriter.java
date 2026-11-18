package java.io;
public class StringWriter extends Writer {
    private final StringBuffer buf = new StringBuffer();
    public StringWriter() {}
    public StringWriter(int n) {}
    public void write(int c) { buf.append((char) c); }
    public void write(char[] c, int off, int len) { buf.append(c, off, len); }
    public void write(String s) { buf.append(s); }
    public StringWriter append(CharSequence s) { buf.append(String.valueOf(s)); return this; }
    public StringWriter append(char c) { buf.append(c); return this; }
    public StringBuffer getBuffer() { return buf; }
    public String toString() { return buf.toString(); }
}
