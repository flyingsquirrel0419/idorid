package java.io;
public abstract class Writer implements Appendable, Closeable, Flushable {
    public void write(int c) throws IOException { write(new char[] { (char) c }, 0, 1); }
    public void write(char[] c) throws IOException { write(c, 0, c.length); }
    public abstract void write(char[] c, int off, int len) throws IOException;
    public void write(String s) throws IOException { write(s.toCharArray(), 0, s.length()); }
    public void write(String s, int off, int len) throws IOException { write(s.substring(off, off + len)); }
    public Writer append(CharSequence s) throws IOException { write(String.valueOf(s)); return this; }
    public Writer append(CharSequence s, int b, int e) throws IOException { write(String.valueOf(s).substring(b, e)); return this; }
    public Writer append(char c) throws IOException { write(c); return this; }
    public void flush() throws IOException {}
    public void close() throws IOException {}
}
