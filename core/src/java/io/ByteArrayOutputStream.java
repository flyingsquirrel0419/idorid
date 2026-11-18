package java.io;
public class ByteArrayOutputStream extends OutputStream {
    protected byte[] buf;
    protected int count;
    public ByteArrayOutputStream() { this(32); }
    public ByteArrayOutputStream(int n) { buf = new byte[n]; }
    public void write(int b) {
        if (count == buf.length) { byte[] n = new byte[buf.length * 2 + 1]; System.arraycopy(buf, 0, n, 0, count); buf = n; }
        buf[count++] = (byte) b;
    }
    public void write(byte[] b, int off, int len) { for (int i = 0; i < len; i++) write(b[off + i]); }
    public byte[] toByteArray() { byte[] r = new byte[count]; System.arraycopy(buf, 0, r, 0, count); return r; }
    public int size() { return count; }
    public void reset() { count = 0; }
    public String toString() { return new String(buf, 0, count); }
}
