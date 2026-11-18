package java.io;
public abstract class InputStream implements Closeable {
    public abstract int read() throws IOException;
    public int read(byte[] b) throws IOException { return read(b, 0, b.length); }
    public int read(byte[] b, int off, int len) throws IOException {
        int i = 0;
        for (; i < len; i++) { int c = read(); if (c < 0) break; b[off + i] = (byte) c; }
        return i == 0 && len > 0 ? -1 : i;
    }
    public int available() throws IOException { return 0; }
    public long skip(long n) throws IOException { long i = 0; while (i < n && read() >= 0) i++; return i; }
    public void close() throws IOException {}
}
